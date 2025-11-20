//! Common Database Queries
//!
//! Pre-defined queries for common operations with optimized execution plans.

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Query to get event count by source module over time
pub async fn get_event_count_by_module(
    pool: &PgPool,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> anyhow::Result<Vec<(String, i64)>> {
    let rows = sqlx::query(
        r#"
        SELECT
            source_module->>'type' as module,
            COUNT(*) as count
        FROM events
        WHERE timestamp >= $1 AND timestamp < $2
        GROUP BY source_module->>'type'
        ORDER BY count DESC
        "#
    )
    .bind(start)
    .bind(end)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter()
        .map(|row| (row.get("module"), row.get("count")))
        .collect())
}

/// Query to get average metric value over time buckets
pub async fn get_metric_timeseries(
    pool: &PgPool,
    metric_name: &str,
    bucket_size: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> anyhow::Result<Vec<(DateTime<Utc>, f64)>> {
    let query_str = format!(
        r#"
        SELECT
            time_bucket('{}', window_start) as bucket,
            AVG(avg) as value
        FROM aggregated_metrics
        WHERE metric_name = $1
          AND window_start >= $2
          AND window_start < $3
        GROUP BY bucket
        ORDER BY bucket ASC
        "#,
        bucket_size
    );

    let rows = sqlx::query(&query_str)
        .bind(metric_name)
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await?;

    Ok(rows.into_iter()
        .map(|row| (row.get("bucket"), row.get("value")))
        .collect())
}

/// Query to find correlated events
pub async fn find_correlated_events(
    pool: &PgPool,
    event_id: Uuid,
    min_strength: f64,
) -> anyhow::Result<Vec<Uuid>> {
    let rows = sqlx::query(
        r#"
        SELECT DISTINCT
            CASE
                WHEN source_event_id = $1 THEN target_event_id
                ELSE source_event_id
            END as related_event_id
        FROM correlations
        WHERE (source_event_id = $1 OR target_event_id = $1)
          AND strength >= $2
        ORDER BY related_event_id
        "#
    )
    .bind(event_id)
    .bind(min_strength)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter()
        .map(|row| row.get("related_event_id"))
        .collect())
}

/// Query to get top anomalies by confidence score
pub async fn get_top_anomalies(
    pool: &PgPool,
    limit: i64,
    min_confidence: f64,
) -> anyhow::Result<Vec<(Uuid, String, f64)>> {
    let rows = sqlx::query(
        r#"
        SELECT anomaly_id, metric_name, confidence_score
        FROM anomalies
        WHERE confidence_score >= $1
        ORDER BY confidence_score DESC, detected_at DESC
        LIMIT $2
        "#
    )
    .bind(min_confidence)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter()
        .map(|row| (
            row.get("anomaly_id"),
            row.get("metric_name"),
            row.get("confidence_score")
        ))
        .collect())
}
