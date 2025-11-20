//! Database Schema Definitions
//!
//! SQL schema definitions for TimescaleDB tables and hypertables.

/// SQL to create the events hypertable
pub const CREATE_EVENTS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS events (
    event_id UUID PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL,
    source_module JSONB NOT NULL,
    event_type JSONB NOT NULL,
    correlation_id UUID,
    parent_event_id UUID,
    schema_version TEXT NOT NULL,
    severity JSONB NOT NULL,
    environment TEXT NOT NULL,
    tags JSONB NOT NULL DEFAULT '{}',
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Convert to hypertable for time-series optimization
SELECT create_hypertable('events', 'timestamp', if_not_exists => TRUE);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events (timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_events_correlation_id ON events (correlation_id) WHERE correlation_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_events_source_module ON events ((source_module->>'type'));
CREATE INDEX IF NOT EXISTS idx_events_event_type ON events ((event_type->>'type'));
CREATE INDEX IF NOT EXISTS idx_events_severity ON events ((severity->>'level'));
CREATE INDEX IF NOT EXISTS idx_events_tags ON events USING GIN (tags);

-- Enable compression (4:1 ratio typical)
ALTER TABLE events SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'source_module, event_type',
    timescaledb.compress_orderby = 'timestamp DESC'
);

-- Compress chunks older than 7 days
SELECT add_compression_policy('events', INTERVAL '7 days', if_not_exists => TRUE);
"#;

/// SQL to create aggregated metrics table
pub const CREATE_AGGREGATED_METRICS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS aggregated_metrics (
    id BIGSERIAL,
    metric_name TEXT NOT NULL,
    time_window TEXT NOT NULL,
    window_start TIMESTAMPTZ NOT NULL,
    tags JSONB NOT NULL DEFAULT '{}',
    avg DOUBLE PRECISION NOT NULL,
    min DOUBLE PRECISION NOT NULL,
    max DOUBLE PRECISION NOT NULL,
    p50 DOUBLE PRECISION NOT NULL,
    p95 DOUBLE PRECISION NOT NULL,
    p99 DOUBLE PRECISION NOT NULL,
    stddev DOUBLE PRECISION,
    count BIGINT NOT NULL,
    sum DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (metric_name, time_window, window_start, tags)
);

-- Convert to hypertable
SELECT create_hypertable('aggregated_metrics', 'window_start', if_not_exists => TRUE);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_aggregated_metrics_metric_window
    ON aggregated_metrics (metric_name, time_window, window_start DESC);
CREATE INDEX IF NOT EXISTS idx_aggregated_metrics_tags
    ON aggregated_metrics USING GIN (tags);

-- Enable compression
ALTER TABLE aggregated_metrics SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'metric_name, time_window',
    timescaledb.compress_orderby = 'window_start DESC'
);

-- Compress chunks older than 30 days
SELECT add_compression_policy('aggregated_metrics', INTERVAL '30 days', if_not_exists => TRUE);

-- Create continuous aggregate for real-time metrics (1-minute window)
CREATE MATERIALIZED VIEW IF NOT EXISTS metrics_1min
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 minute', timestamp) AS bucket,
    source_module,
    event_type,
    COUNT(*) as event_count,
    AVG((payload->>'value')::DOUBLE PRECISION) as avg_value
FROM events
WHERE payload->>'value' IS NOT NULL
GROUP BY bucket, source_module, event_type
WITH NO DATA;

-- Refresh policy for continuous aggregate
SELECT add_continuous_aggregate_policy('metrics_1min',
    start_offset => INTERVAL '1 hour',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute',
    if_not_exists => TRUE);
"#;

/// SQL to create anomalies table
pub const CREATE_ANOMALIES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS anomalies (
    anomaly_id UUID PRIMARY KEY,
    detected_at TIMESTAMPTZ NOT NULL,
    metric_name TEXT NOT NULL,
    anomaly_type TEXT NOT NULL,
    severity TEXT NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    expected_value DOUBLE PRECISION,
    confidence_score DOUBLE PRECISION NOT NULL,
    context JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Convert to hypertable
SELECT create_hypertable('anomalies', 'detected_at', if_not_exists => TRUE);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_anomalies_detected_at ON anomalies (detected_at DESC);
CREATE INDEX IF NOT EXISTS idx_anomalies_metric_name ON anomalies (metric_name);
CREATE INDEX IF NOT EXISTS idx_anomalies_severity ON anomalies (severity);
CREATE INDEX IF NOT EXISTS idx_anomalies_type ON anomalies (anomaly_type);
"#;

/// SQL to create correlations table
pub const CREATE_CORRELATIONS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS correlations (
    correlation_id UUID PRIMARY KEY,
    correlation_type TEXT NOT NULL,
    source_event_id UUID NOT NULL,
    target_event_id UUID NOT NULL,
    strength DOUBLE PRECISION NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (source_event_id) REFERENCES events(event_id),
    FOREIGN KEY (target_event_id) REFERENCES events(event_id)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_correlations_source ON correlations (source_event_id);
CREATE INDEX IF NOT EXISTS idx_correlations_target ON correlations (target_event_id);
CREATE INDEX IF NOT EXISTS idx_correlations_type ON correlations (correlation_type);
CREATE INDEX IF NOT EXISTS idx_correlations_strength ON correlations (strength DESC);
"#;

/// SQL to create retention policies
pub const CREATE_RETENTION_POLICIES: &str = r#"
-- Retention policy for events: keep raw events for 30 days
SELECT add_retention_policy('events', INTERVAL '30 days', if_not_exists => TRUE);

-- Retention policy for aggregated metrics: keep for 365 days
SELECT add_retention_policy('aggregated_metrics', INTERVAL '365 days', if_not_exists => TRUE);

-- Retention policy for anomalies: keep for 90 days
SELECT add_retention_policy('anomalies', INTERVAL '90 days', if_not_exists => TRUE);

-- Correlations don't have retention (or set very long, e.g., 2 years)
"#;

/// Initialize all database schemas
pub async fn initialize_schema(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    // Create TimescaleDB extension
    sqlx::query("CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE")
        .execute(pool)
        .await?;

    // Create tables
    sqlx::query(CREATE_EVENTS_TABLE).execute(pool).await?;
    sqlx::query(CREATE_AGGREGATED_METRICS_TABLE).execute(pool).await?;
    sqlx::query(CREATE_ANOMALIES_TABLE).execute(pool).await?;
    sqlx::query(CREATE_CORRELATIONS_TABLE).execute(pool).await?;

    // Create retention policies
    sqlx::query(CREATE_RETENTION_POLICIES).execute(pool).await?;

    Ok(())
}
