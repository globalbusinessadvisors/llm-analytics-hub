//! Database Interaction Layer
//!
//! Provides high-performance database operations with connection pooling,
//! prepared statements, and transaction management for TimescaleDB.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions, PgQueryResult};
use sqlx::{query, query_as, FromRow, Row};
use std::time::Duration;
use tracing::{info, instrument, warn};
use uuid::Uuid;

pub mod queries;
pub mod schema;

use crate::schemas::events::AnalyticsEvent;
use crate::models::metrics::{StatisticalMeasures, TimeWindow};

/// Database configuration
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            database: "llm_analytics".to_string(),
            username: "postgres".to_string(),
            password: "postgres".to_string(),
            max_connections: 50,
            min_connections: 5,
            connection_timeout: 30,
            idle_timeout: 600,
            max_lifetime: 1800,
        }
    }
}

impl DatabaseConfig {
    /// Build a PostgreSQL connection string
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

/// Database client with connection pooling
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database client with connection pool
    #[instrument(skip(config))]
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        info!("Initializing database connection pool");

        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(config.connection_timeout))
            .idle_timeout(Duration::from_secs(config.idle_timeout))
            .max_lifetime(Duration::from_secs(config.max_lifetime))
            .connect(&config.connection_string())
            .await
            .context("Failed to create database connection pool")?;

        // Test the connection
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .context("Failed to test database connection")?;

        info!("Database connection pool initialized successfully");

        Ok(Self { pool })
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Close the database connection pool
    pub async fn close(&self) {
        self.pool.close().await;
    }

    // ========== Event Operations ==========

    /// Insert a single analytics event
    #[instrument(skip(self, event))]
    pub async fn insert_event(&self, event: &AnalyticsEvent) -> Result<Uuid> {
        let event_json = serde_json::to_value(event)
            .context("Failed to serialize event")?;

        let event_id = query!(
            r#"
            INSERT INTO events (
                event_id, timestamp, source_module, event_type,
                correlation_id, parent_event_id, schema_version,
                severity, environment, tags, payload
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING event_id
            "#,
            event.common.event_id,
            event.common.timestamp,
            serde_json::to_value(&event.common.source_module)?,
            serde_json::to_value(&event.common.event_type)?,
            event.common.correlation_id,
            event.common.parent_event_id,
            event.common.schema_version,
            serde_json::to_value(&event.common.severity)?,
            event.common.environment,
            serde_json::to_value(&event.common.tags)?,
            event_json
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to insert event")?;

        Ok(event_id.event_id)
    }

    /// Batch insert analytics events for high throughput
    #[instrument(skip(self, events))]
    pub async fn insert_events_batch(&self, events: &[AnalyticsEvent]) -> Result<u64> {
        if events.is_empty() {
            return Ok(0);
        }

        let mut tx = self.pool.begin().await?;
        let mut inserted = 0u64;

        // Use COPY for maximum performance with large batches
        if events.len() > 100 {
            // Build bulk insert query
            let mut query_builder = sqlx::QueryBuilder::new(
                "INSERT INTO events (event_id, timestamp, source_module, event_type, \
                 correlation_id, parent_event_id, schema_version, severity, environment, \
                 tags, payload) "
            );

            query_builder.push_values(events, |mut b, event| {
                let event_json = serde_json::to_value(event).unwrap();
                b.push_bind(event.common.event_id)
                    .push_bind(event.common.timestamp)
                    .push_bind(serde_json::to_value(&event.common.source_module).unwrap())
                    .push_bind(serde_json::to_value(&event.common.event_type).unwrap())
                    .push_bind(event.common.correlation_id)
                    .push_bind(event.common.parent_event_id)
                    .push_bind(&event.common.schema_version)
                    .push_bind(serde_json::to_value(&event.common.severity).unwrap())
                    .push_bind(&event.common.environment)
                    .push_bind(serde_json::to_value(&event.common.tags).unwrap())
                    .push_bind(event_json);
            });

            let result = query_builder.build().execute(&mut *tx).await?;
            inserted = result.rows_affected();
        } else {
            // Use individual inserts for smaller batches
            for event in events {
                self.insert_event(event).await?;
                inserted += 1;
            }
        }

        tx.commit().await?;
        Ok(inserted)
    }

    /// Query events by time range
    #[instrument(skip(self))]
    pub async fn query_events(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        limit: Option<i64>,
    ) -> Result<Vec<AnalyticsEvent>> {
        let limit = limit.unwrap_or(1000);

        let rows = query!(
            r#"
            SELECT payload
            FROM events
            WHERE timestamp >= $1 AND timestamp < $2
            ORDER BY timestamp DESC
            LIMIT $3
            "#,
            start,
            end,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to query events")?;

        let events: Vec<AnalyticsEvent> = rows
            .into_iter()
            .filter_map(|row| serde_json::from_value(row.payload).ok())
            .collect();

        Ok(events)
    }

    /// Query events by correlation ID
    #[instrument(skip(self))]
    pub async fn query_events_by_correlation(
        &self,
        correlation_id: Uuid,
    ) -> Result<Vec<AnalyticsEvent>> {
        let rows = query!(
            r#"
            SELECT payload
            FROM events
            WHERE correlation_id = $1
            ORDER BY timestamp ASC
            "#,
            correlation_id
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to query events by correlation")?;

        let events: Vec<AnalyticsEvent> = rows
            .into_iter()
            .filter_map(|row| serde_json::from_value(row.payload).ok())
            .collect();

        Ok(events)
    }

    // ========== Metrics Operations ==========

    /// Store aggregated metrics
    #[instrument(skip(self))]
    pub async fn store_aggregated_metric(
        &self,
        metric_name: &str,
        time_window: TimeWindow,
        window_start: DateTime<Utc>,
        tags: &serde_json::Value,
        measures: &StatisticalMeasures,
    ) -> Result<()> {
        query!(
            r#"
            INSERT INTO aggregated_metrics (
                metric_name, time_window, window_start, tags,
                avg, min, max, p50, p95, p99, stddev, count, sum
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT (metric_name, time_window, window_start, tags)
            DO UPDATE SET
                avg = EXCLUDED.avg,
                min = EXCLUDED.min,
                max = EXCLUDED.max,
                p50 = EXCLUDED.p50,
                p95 = EXCLUDED.p95,
                p99 = EXCLUDED.p99,
                stddev = EXCLUDED.stddev,
                count = EXCLUDED.count,
                sum = EXCLUDED.sum
            "#,
            metric_name,
            time_window.as_str(),
            window_start,
            tags,
            measures.avg,
            measures.min,
            measures.max,
            measures.p50,
            measures.p95,
            measures.p99,
            measures.stddev,
            measures.count as i64,
            measures.sum
        )
        .execute(&self.pool)
        .await
        .context("Failed to store aggregated metric")?;

        Ok(())
    }

    /// Query aggregated metrics
    #[instrument(skip(self))]
    pub async fn query_aggregated_metrics(
        &self,
        metric_name: &str,
        time_window: TimeWindow,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<AggregatedMetricRow>> {
        let rows = query_as!(
            AggregatedMetricRow,
            r#"
            SELECT
                metric_name, time_window, window_start, tags,
                avg, min, max, p50, p95, p99, stddev, count, sum
            FROM aggregated_metrics
            WHERE metric_name = $1
              AND time_window = $2
              AND window_start >= $3
              AND window_start < $4
            ORDER BY window_start ASC
            "#,
            metric_name,
            time_window.as_str(),
            start,
            end
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to query aggregated metrics")?;

        Ok(rows)
    }

    // ========== Anomaly Operations ==========

    /// Store detected anomaly
    #[instrument(skip(self))]
    pub async fn store_anomaly(
        &self,
        anomaly_id: Uuid,
        detected_at: DateTime<Utc>,
        metric_name: &str,
        anomaly_type: &str,
        severity: &str,
        value: f64,
        expected_value: Option<f64>,
        confidence_score: f64,
        context: &serde_json::Value,
    ) -> Result<Uuid> {
        let result = query!(
            r#"
            INSERT INTO anomalies (
                anomaly_id, detected_at, metric_name, anomaly_type,
                severity, value, expected_value, confidence_score, context
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING anomaly_id
            "#,
            anomaly_id,
            detected_at,
            metric_name,
            anomaly_type,
            severity,
            value,
            expected_value,
            confidence_score,
            context
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to store anomaly")?;

        Ok(result.anomaly_id)
    }

    /// Query recent anomalies
    #[instrument(skip(self))]
    pub async fn query_recent_anomalies(
        &self,
        since: DateTime<Utc>,
        limit: Option<i64>,
    ) -> Result<Vec<AnomalyRow>> {
        let limit = limit.unwrap_or(100);

        let rows = query_as!(
            AnomalyRow,
            r#"
            SELECT
                anomaly_id, detected_at, metric_name, anomaly_type,
                severity, value, expected_value, confidence_score, context
            FROM anomalies
            WHERE detected_at >= $1
            ORDER BY detected_at DESC
            LIMIT $2
            "#,
            since,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to query anomalies")?;

        Ok(rows)
    }

    // ========== Correlation Operations ==========

    /// Store event correlation
    #[instrument(skip(self))]
    pub async fn store_correlation(
        &self,
        correlation_id: Uuid,
        correlation_type: &str,
        source_event_id: Uuid,
        target_event_id: Uuid,
        strength: f64,
        metadata: &serde_json::Value,
    ) -> Result<Uuid> {
        let result = query!(
            r#"
            INSERT INTO correlations (
                correlation_id, correlation_type, source_event_id,
                target_event_id, strength, metadata, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
            RETURNING correlation_id
            "#,
            correlation_id,
            correlation_type,
            source_event_id,
            target_event_id,
            strength,
            metadata
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to store correlation")?;

        Ok(result.correlation_id)
    }

    // ========== Health Check ==========

    /// Check database health
    pub async fn health_check(&self) -> Result<DatabaseHealth> {
        let result = query!(
            r#"
            SELECT
                (SELECT COUNT(*) FROM pg_stat_activity WHERE state = 'active') as active_connections,
                (SELECT COUNT(*) FROM events WHERE timestamp > NOW() - INTERVAL '1 minute') as recent_events,
                (SELECT pg_database_size(current_database())) as database_size
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(DatabaseHealth {
            active_connections: result.active_connections.unwrap_or(0),
            recent_events: result.recent_events.unwrap_or(0),
            database_size_bytes: result.database_size.unwrap_or(0),
        })
    }
}

// ========== Database Types ==========

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AggregatedMetricRow {
    pub metric_name: String,
    pub time_window: String,
    pub window_start: DateTime<Utc>,
    pub tags: serde_json::Value,
    pub avg: f64,
    pub min: f64,
    pub max: f64,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
    pub stddev: Option<f64>,
    pub count: i64,
    pub sum: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AnomalyRow {
    pub anomaly_id: Uuid,
    pub detected_at: DateTime<Utc>,
    pub metric_name: String,
    pub anomaly_type: String,
    pub severity: String,
    pub value: f64,
    pub expected_value: Option<f64>,
    pub confidence_score: f64,
    pub context: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseHealth {
    pub active_connections: i64,
    pub recent_events: i64,
    pub database_size_bytes: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_config_connection_string() {
        let config = DatabaseConfig::default();
        let conn_str = config.connection_string();
        assert!(conn_str.contains("postgres://"));
        assert!(conn_str.contains("localhost:5432"));
    }
}
