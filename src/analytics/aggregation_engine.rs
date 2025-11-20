//! Time-Series Aggregation Engine
//!
//! High-performance aggregation of events into statistical measures across
//! multiple time windows (1m, 5m, 15m, 1h, 6h, 1d, 1w, 1M).

use crate::database::Database;
use crate::models::metrics::{StatisticalMeasures, TimeWindow};
use crate::schemas::events::AnalyticsEvent;
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, instrument};

/// Aggregation engine for time-series data
pub struct AggregationEngine {
    database: Arc<Database>,
    // Metric name + window -> Aggregated data
    aggregates: Arc<DashMap<AggregateKey, WindowedAggregates>>,
    // Active time buckets
    active_buckets: Arc<RwLock<HashMap<TimeWindow, DateTime<Utc>>>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct AggregateKey {
    metric_name: String,
    window: TimeWindow,
    tags_hash: u64,
}

impl AggregationEngine {
    /// Create a new aggregation engine
    pub fn new(database: Arc<Database>) -> Self {
        Self {
            database,
            aggregates: Arc::new(DashMap::new()),
            active_buckets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Process an event and update aggregations
    #[instrument(skip(self, event))]
    pub async fn process_event(&self, event: &AnalyticsEvent) -> Result<()> {
        // Extract numeric metrics from the event
        let metrics = self.extract_metrics(event)?;

        for (metric_name, value) in metrics {
            // Aggregate across all time windows
            for window in Self::all_windows() {
                self.update_aggregation(
                    &metric_name,
                    value,
                    window,
                    event.common.timestamp,
                    &event.common.tags,
                )
                .await?;
            }
        }

        Ok(())
    }

    /// Extract numeric metrics from an event
    fn extract_metrics(&self, event: &AnalyticsEvent) -> Result<Vec<(String, f64)>> {
        let mut metrics = Vec::new();

        // This is a simplified extraction - in reality, you'd parse the payload
        // based on event type and extract relevant metrics
        if let Ok(value) = serde_json::to_value(&event.payload) {
            if let Some(obj) = value.as_object() {
                for (key, val) in obj {
                    if let Some(num) = val.as_f64() {
                        metrics.push((key.clone(), num));
                    }
                }
            }
        }

        Ok(metrics)
    }

    /// Update aggregation for a specific metric and time window
    async fn update_aggregation(
        &self,
        metric_name: &str,
        value: f64,
        window: TimeWindow,
        timestamp: DateTime<Utc>,
        tags: &HashMap<String, String>,
    ) -> Result<()> {
        let window_start = self.align_to_window(timestamp, window);
        let tags_hash = self.hash_tags(tags);

        let key = AggregateKey {
            metric_name: metric_name.to_string(),
            window,
            tags_hash,
        };

        // Get or create windowed aggregates
        let mut agg = self.aggregates
            .entry(key.clone())
            .or_insert_with(|| WindowedAggregates::new(window_start));

        // Add value to current window
        agg.add_value(value);

        // Check if we need to flush this window
        if timestamp >= agg.window_end() {
            // Compute statistics and store
            let measures = agg.compute_statistics();

            // Store to database
            let tags_json = serde_json::to_value(tags)?;
            self.database
                .store_aggregated_metric(
                    metric_name,
                    window,
                    agg.window_start,
                    &tags_json,
                    &measures,
                )
                .await?;

            debug!(
                "Flushed {} window for {}: avg={:.2}, count={}",
                window.as_str(),
                metric_name,
                measures.avg,
                measures.count
            );

            // Create new window
            *agg = WindowedAggregates::new(window_start);
        }

        Ok(())
    }

    /// Align timestamp to window boundary
    fn align_to_window(&self, timestamp: DateTime<Utc>, window: TimeWindow) -> DateTime<Utc> {
        let seconds = window.to_seconds() as i64;
        let ts = timestamp.timestamp();
        let aligned = (ts / seconds) * seconds;
        DateTime::from_timestamp(aligned, 0).unwrap_or(timestamp)
    }

    /// Hash tags for deduplication
    fn hash_tags(&self, tags: &HashMap<String, String>) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        let mut sorted: Vec<_> = tags.iter().collect();
        sorted.sort_by_key(|(k, _)| *k);
        sorted.hash(&mut hasher);
        hasher.finish()
    }

    /// All supported time windows
    fn all_windows() -> &'static [TimeWindow] {
        &[
            TimeWindow::OneMinute,
            TimeWindow::FiveMinutes,
            TimeWindow::FifteenMinutes,
            TimeWindow::OneHour,
            TimeWindow::SixHours,
            TimeWindow::OneDay,
            TimeWindow::OneWeek,
            TimeWindow::OneMonth,
        ]
    }

    /// Get aggregated statistics for a metric
    #[instrument(skip(self))]
    pub async fn get_aggregated_stats(
        &self,
        metric_name: &str,
        window: TimeWindow,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<StatisticalMeasures>> {
        let rows = self.database
            .query_aggregated_metrics(metric_name, window, start, end)
            .await?;

        let stats: Vec<StatisticalMeasures> = rows
            .into_iter()
            .map(|row| StatisticalMeasures {
                avg: row.avg,
                min: row.min,
                max: row.max,
                p50: row.p50,
                p95: row.p95,
                p99: row.p99,
                stddev: row.stddev,
                count: row.count as u64,
                sum: row.sum,
            })
            .collect();

        Ok(stats)
    }

    /// Force flush all pending aggregates
    pub async fn flush_all(&self) -> Result<usize> {
        let mut flushed = 0;

        for entry in self.aggregates.iter() {
            let key = entry.key();
            let agg = entry.value();

            if agg.values.is_empty() {
                continue;
            }

            let measures = agg.compute_statistics();
            let tags_json = serde_json::json!({});

            self.database
                .store_aggregated_metric(
                    &key.metric_name,
                    key.window,
                    agg.window_start,
                    &tags_json,
                    &measures,
                )
                .await?;

            flushed += 1;
        }

        info!("Flushed {} pending aggregates", flushed);
        Ok(flushed)
    }
}

/// Windowed aggregates for a metric
struct WindowedAggregates {
    window_start: DateTime<Utc>,
    window_duration: Duration,
    values: Vec<f64>,
}

impl WindowedAggregates {
    fn new(window_start: DateTime<Utc>) -> Self {
        Self {
            window_start,
            window_duration: Duration::minutes(1),
            values: Vec::new(),
        }
    }

    fn add_value(&mut self, value: f64) {
        self.values.push(value);
    }

    fn window_end(&self) -> DateTime<Utc> {
        self.window_start + self.window_duration
    }

    fn compute_statistics(&self) -> StatisticalMeasures {
        if self.values.is_empty() {
            return StatisticalMeasures::default();
        }

        let mut sorted = self.values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let count = sorted.len() as u64;
        let sum: f64 = sorted.iter().sum();
        let avg = sum / count as f64;

        let min = sorted[0];
        let max = sorted[sorted.len() - 1];

        // Percentiles
        let p50 = Self::percentile(&sorted, 50.0);
        let p95 = Self::percentile(&sorted, 95.0);
        let p99 = Self::percentile(&sorted, 99.0);

        // Standard deviation
        let variance: f64 = sorted
            .iter()
            .map(|v| {
                let diff = v - avg;
                diff * diff
            })
            .sum::<f64>() / count as f64;
        let stddev = Some(variance.sqrt());

        StatisticalMeasures {
            avg,
            min,
            max,
            p50,
            p95,
            p99,
            stddev,
            count,
            sum,
        }
    }

    fn percentile(sorted_values: &[f64], percentile: f64) -> f64 {
        if sorted_values.is_empty() {
            return 0.0;
        }

        let index = (percentile / 100.0 * (sorted_values.len() - 1) as f64) as usize;
        sorted_values[index.min(sorted_values.len() - 1)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentile_calculation() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        assert_eq!(WindowedAggregates::percentile(&values, 50.0), 5.0);
        assert_eq!(WindowedAggregates::percentile(&values, 95.0), 10.0);
    }

    #[test]
    fn test_statistical_measures() {
        let mut agg = WindowedAggregates::new(Utc::now());
        for i in 1..=10 {
            agg.add_value(i as f64);
        }

        let stats = agg.compute_statistics();
        assert_eq!(stats.count, 10);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 10.0);
        assert_eq!(stats.avg, 5.5);
    }
}
