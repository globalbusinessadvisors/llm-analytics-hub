//! Metrics Aggregation Models
//!
//! Time-window aggregations, statistical measures, and metric types for analytics.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Time window for metric aggregation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum TimeWindow {
    /// 1 minute aggregation
    OneMinute,
    /// 5 minute aggregation
    FiveMinutes,
    /// 15 minute aggregation
    FifteenMinutes,
    /// 1 hour aggregation
    OneHour,
    /// 6 hour aggregation
    SixHours,
    /// 1 day aggregation
    OneDay,
    /// 1 week aggregation
    OneWeek,
    /// 1 month aggregation
    OneMonth,
}

impl TimeWindow {
    /// Returns the duration in seconds
    pub fn to_seconds(&self) -> u64 {
        match self {
            TimeWindow::OneMinute => 60,
            TimeWindow::FiveMinutes => 300,
            TimeWindow::FifteenMinutes => 900,
            TimeWindow::OneHour => 3600,
            TimeWindow::SixHours => 21600,
            TimeWindow::OneDay => 86400,
            TimeWindow::OneWeek => 604800,
            TimeWindow::OneMonth => 2592000, // 30 days
        }
    }

    /// Returns human-readable string
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeWindow::OneMinute => "1m",
            TimeWindow::FiveMinutes => "5m",
            TimeWindow::FifteenMinutes => "15m",
            TimeWindow::OneHour => "1h",
            TimeWindow::SixHours => "6h",
            TimeWindow::OneDay => "1d",
            TimeWindow::OneWeek => "1w",
            TimeWindow::OneMonth => "1M",
        }
    }
}

/// Statistical measures for metric aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalMeasures {
    /// Average value
    pub avg: f64,

    /// Minimum value
    pub min: f64,

    /// Maximum value
    pub max: f64,

    /// Median (50th percentile)
    pub p50: f64,

    /// 95th percentile
    pub p95: f64,

    /// 99th percentile
    pub p99: f64,

    /// Standard deviation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stddev: Option<f64>,

    /// Sample count
    pub count: u64,

    /// Sum of all values
    pub sum: f64,
}

impl Default for StatisticalMeasures {
    fn default() -> Self {
        Self {
            avg: 0.0,
            min: 0.0,
            max: 0.0,
            p50: 0.0,
            p95: 0.0,
            p99: 0.0,
            stddev: None,
            count: 0,
            sum: 0.0,
        }
    }
}

/// Base metric types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "metric_type")]
pub enum MetricType {
    /// Counter - monotonically increasing value
    #[serde(rename = "counter")]
    Counter(CounterMetric),

    /// Gauge - point-in-time value that can go up or down
    #[serde(rename = "gauge")]
    Gauge(GaugeMetric),

    /// Histogram - distribution of values with statistical measures
    #[serde(rename = "histogram")]
    Histogram(HistogramMetric),

    /// Summary - similar to histogram but with configurable percentiles
    #[serde(rename = "summary")]
    Summary(SummaryMetric),
}

/// Counter metric - monotonically increasing value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterMetric {
    /// Metric name
    pub name: String,

    /// Current counter value
    pub value: u64,

    /// Rate of change (per second)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,

    /// Tags for filtering and grouping
    #[serde(default)]
    pub tags: HashMap<String, String>,

    /// Timestamp of the metric
    pub timestamp: DateTime<Utc>,
}

/// Gauge metric - point-in-time value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaugeMetric {
    /// Metric name
    pub name: String,

    /// Current gauge value
    pub value: f64,

    /// Previous value for delta calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_value: Option<f64>,

    /// Tags for filtering and grouping
    #[serde(default)]
    pub tags: HashMap<String, String>,

    /// Timestamp of the metric
    pub timestamp: DateTime<Utc>,
}

/// Histogram metric - distribution of values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramMetric {
    /// Metric name
    pub name: String,

    /// Statistical measures
    pub stats: StatisticalMeasures,

    /// Histogram buckets with counts
    pub buckets: Vec<HistogramBucket>,

    /// Tags for filtering and grouping
    #[serde(default)]
    pub tags: HashMap<String, String>,

    /// Timestamp of the metric
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramBucket {
    /// Upper bound of the bucket (inclusive)
    pub upper_bound: f64,

    /// Count of values in this bucket
    pub count: u64,
}

/// Summary metric - similar to histogram with explicit percentiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryMetric {
    /// Metric name
    pub name: String,

    /// Statistical measures
    pub stats: StatisticalMeasures,

    /// Additional custom percentiles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentiles: Option<HashMap<String, f64>>,

    /// Tags for filtering and grouping
    #[serde(default)]
    pub tags: HashMap<String, String>,

    /// Timestamp of the metric
    pub timestamp: DateTime<Utc>,
}

/// Aggregated metric over a time window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetric {
    /// Metric name
    pub name: String,

    /// Time window for aggregation
    pub window: TimeWindow,

    /// Start time of the window
    pub window_start: DateTime<Utc>,

    /// End time of the window
    pub window_end: DateTime<Utc>,

    /// Aggregated values
    pub values: MetricValues,

    /// Tags for filtering and grouping
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetricValues {
    /// Statistical distribution
    Stats(StatisticalMeasures),

    /// Counter value and rate
    Counter {
        value: u64,
        rate: f64,
    },

    /// Gauge value
    Gauge {
        value: f64,
        delta: Option<f64>,
    },
}

/// Composite metric combining multiple module metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeMetric {
    /// Composite metric identifier
    pub metric_id: String,

    /// Human-readable name
    pub name: String,

    /// Description of what this metric represents
    pub description: String,

    /// Source modules contributing to this metric
    pub source_modules: Vec<String>,

    /// Component metrics
    pub components: Vec<ComponentMetric>,

    /// Computed composite value
    pub value: f64,

    /// Computation formula (e.g., "cost_per_request = total_cost / request_count")
    pub formula: String,

    /// Time window
    pub window: TimeWindow,

    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetric {
    pub name: String,
    pub source_module: String,
    pub value: f64,
    pub weight: Option<f64>,
}

/// Pre-defined composite metrics across modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossModuleMetrics {
    /// Cost per request (CostOps + Observatory)
    pub cost_per_request: Option<f64>,

    /// Error-adjusted throughput (Observatory + Sentinel)
    pub error_adjusted_throughput: Option<f64>,

    /// Compliance-weighted performance (Observatory + Governance)
    pub compliance_weighted_performance: Option<f64>,

    /// Security-adjusted cost efficiency (CostOps + Sentinel)
    pub security_adjusted_cost_efficiency: Option<f64>,

    /// Overall system health score (all modules)
    pub system_health_score: Option<f64>,
}

/// Metric query specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricQuery {
    /// Metric name or pattern
    pub metric_name: String,

    /// Time range for the query
    pub time_range: TimeRange,

    /// Aggregation window
    pub window: Option<TimeWindow>,

    /// Tags for filtering
    #[serde(default)]
    pub tag_filters: HashMap<String, String>,

    /// Statistical measures to include
    #[serde(default)]
    pub include_stats: Vec<StatType>,

    /// Group by tags
    #[serde(default)]
    pub group_by: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StatType {
    Avg,
    Min,
    Max,
    P50,
    P95,
    P99,
    Stddev,
    Count,
    Sum,
    Rate,
}

/// Metric rollup configuration for data retention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricRollup {
    /// Source resolution
    pub source_window: TimeWindow,

    /// Target resolution
    pub target_window: TimeWindow,

    /// Aggregation function to apply
    pub aggregation: AggregationFunction,

    /// Retention period for rolled-up data (in days)
    pub retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AggregationFunction {
    Avg,
    Sum,
    Min,
    Max,
    Last,
    First,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_window_conversions() {
        assert_eq!(TimeWindow::OneMinute.to_seconds(), 60);
        assert_eq!(TimeWindow::OneHour.to_seconds(), 3600);
        assert_eq!(TimeWindow::OneDay.to_seconds(), 86400);
        assert_eq!(TimeWindow::OneMinute.as_str(), "1m");
    }

    #[test]
    fn test_counter_metric_serialization() {
        let mut tags = HashMap::new();
        tags.insert("model".to_string(), "gpt-4".to_string());

        let counter = MetricType::Counter(CounterMetric {
            name: "requests_total".to_string(),
            value: 12345,
            rate: Some(10.5),
            tags,
            timestamp: Utc::now(),
        });

        let json = serde_json::to_string_pretty(&counter).unwrap();
        assert!(json.contains("counter"));
        assert!(json.contains("requests_total"));
    }

    #[test]
    fn test_histogram_metric() {
        let buckets = vec![
            HistogramBucket {
                upper_bound: 100.0,
                count: 50,
            },
            HistogramBucket {
                upper_bound: 500.0,
                count: 30,
            },
            HistogramBucket {
                upper_bound: 1000.0,
                count: 20,
            },
        ];

        let histogram = MetricType::Histogram(HistogramMetric {
            name: "request_latency_ms".to_string(),
            stats: StatisticalMeasures {
                avg: 450.5,
                min: 50.0,
                max: 980.0,
                p50: 420.0,
                p95: 850.0,
                p99: 950.0,
                stddev: Some(150.5),
                count: 100,
                sum: 45050.0,
            },
            buckets,
            tags: HashMap::new(),
            timestamp: Utc::now(),
        });

        let json = serde_json::to_string_pretty(&histogram).unwrap();
        assert!(json.contains("histogram"));
        assert!(json.contains("request_latency_ms"));
    }
}
