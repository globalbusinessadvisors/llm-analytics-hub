//! Time-Series Data Models
//!
//! Tag-based organization, field selection, retention policies, and indexing strategies
//! for efficient time-series data storage and querying.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Time-series data point with tags and fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    /// Measurement name (e.g., "llm_latency", "security_events")
    pub measurement: String,

    /// Timestamp of the data point
    pub timestamp: DateTime<Utc>,

    /// Tags for indexing and filtering (low cardinality, indexed)
    pub tags: TagSet,

    /// Fields containing actual measurements (high cardinality, not indexed)
    pub fields: FieldSet,

    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Tag set for organizing and filtering time-series data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TagSet {
    /// Source module that generated this data
    pub source_module: String,

    /// Environment (production, staging, development)
    pub environment: String,

    /// Region or datacenter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Model identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,

    /// Application or service name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// Version or deployment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Custom tags (keep low cardinality)
    #[serde(flatten)]
    pub custom: HashMap<String, String>,
}

/// Field set containing measurement values
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldSet {
    /// Performance metrics
    Performance(PerformanceFields),

    /// Security metrics
    Security(SecurityFields),

    /// Cost metrics
    Cost(CostFields),

    /// Governance metrics
    Governance(GovernanceFields),

    /// Generic fields
    Generic(HashMap<String, FieldValue>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceFields {
    /// Latency in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<f64>,

    /// Throughput (requests per second)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<f64>,

    /// Error count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<u64>,

    /// Success count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<u64>,

    /// Token count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_count: Option<u64>,

    /// Custom performance metrics
    #[serde(flatten)]
    pub custom: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFields {
    /// Threat count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_count: Option<u64>,

    /// Threat severity score
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_score: Option<f64>,

    /// Blocked requests count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_count: Option<u64>,

    /// Vulnerability count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_count: Option<u64>,

    /// Custom security metrics
    #[serde(flatten)]
    pub custom: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostFields {
    /// Total cost in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_usd: Option<f64>,

    /// Token cost
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_cost: Option<f64>,

    /// Resource utilization percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_percent: Option<f64>,

    /// Custom cost metrics
    #[serde(flatten)]
    pub custom: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceFields {
    /// Policy violation count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_count: Option<u64>,

    /// Compliance score
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_score: Option<f64>,

    /// Audit event count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_count: Option<u64>,

    /// Custom governance metrics
    #[serde(flatten)]
    pub custom: HashMap<String, f64>,
}

/// Field value types supported in time-series data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldValue {
    Float(f64),
    Integer(i64),
    UnsignedInteger(u64),
    String(String),
    Boolean(bool),
}

/// Time-series batch for efficient bulk operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesBatch {
    /// Batch identifier
    pub batch_id: String,

    /// Measurement name (all points in batch have same measurement)
    pub measurement: String,

    /// Data points in this batch
    pub points: Vec<TimeSeriesPoint>,

    /// Batch timestamp
    pub created_at: DateTime<Utc>,
}

/// Retention policy for time-series data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Policy name
    pub name: String,

    /// Duration to keep data at full resolution (in days)
    pub full_resolution_days: u32,

    /// Downsampling configurations
    pub downsample_configs: Vec<DownsampleConfig>,

    /// Maximum retention period (in days)
    pub max_retention_days: u32,

    /// Shard duration for organizing data
    pub shard_duration_hours: u32,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            full_resolution_days: 7,
            downsample_configs: vec![
                DownsampleConfig {
                    after_days: 7,
                    resolution_minutes: 5,
                },
                DownsampleConfig {
                    after_days: 30,
                    resolution_minutes: 60,
                },
                DownsampleConfig {
                    after_days: 90,
                    resolution_minutes: 1440, // daily
                },
            ],
            max_retention_days: 365,
            shard_duration_hours: 24,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownsampleConfig {
    /// Apply downsampling after this many days
    pub after_days: u32,

    /// Target resolution in minutes
    pub resolution_minutes: u32,
}

/// Index configuration for query optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConfig {
    /// Measurement name
    pub measurement: String,

    /// Tags to index (ordered by cardinality, low to high)
    pub indexed_tags: Vec<String>,

    /// Shard key tags for distribution
    pub shard_keys: Vec<String>,

    /// Enable time-based partitioning
    pub time_partitioning: bool,

    /// Partition interval in hours
    pub partition_interval_hours: u32,
}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {
            measurement: String::new(),
            indexed_tags: vec![
                "source_module".to_string(),
                "environment".to_string(),
                "model_id".to_string(),
            ],
            shard_keys: vec!["source_module".to_string(), "environment".to_string()],
            time_partitioning: true,
            partition_interval_hours: 24,
        }
    }
}

/// Time-series query specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesQuery {
    /// Measurement to query
    pub measurement: String,

    /// Time range
    pub time_range: TimeRange,

    /// Tag filters (AND conditions)
    #[serde(default)]
    pub tag_filters: HashMap<String, String>,

    /// Fields to select (empty = all fields)
    #[serde(default)]
    pub select_fields: Vec<String>,

    /// Aggregation function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<Aggregation>,

    /// Group by tags
    #[serde(default)]
    pub group_by: Vec<String>,

    /// Fill strategy for missing data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<FillStrategy>,

    /// Limit number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregation {
    /// Aggregation function
    pub function: AggregationFunction,

    /// Window size for aggregation
    pub window: String, // e.g., "1m", "5m", "1h"

    /// Fields to aggregate
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AggregationFunction {
    Mean,
    Sum,
    Min,
    Max,
    Count,
    First,
    Last,
    Stddev,
    Median,
    Percentile(u8), // e.g., Percentile(95)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FillStrategy {
    /// Fill with null/none
    Null,
    /// Fill with previous value
    Previous,
    /// Fill with linear interpolation
    Linear,
    /// Fill with zero
    Zero,
    /// Fill with specific value
    Value(i64),
}

/// Time-series query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesResult {
    /// Measurement name
    pub measurement: String,

    /// Result series (one per unique tag combination)
    pub series: Vec<TimeSeries>,

    /// Query metadata
    pub metadata: QueryMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeries {
    /// Tag values for this series
    pub tags: HashMap<String, String>,

    /// Data points in chronological order
    pub points: Vec<DataPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Field values
    pub values: HashMap<String, FieldValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMetadata {
    /// Query execution time in milliseconds
    pub execution_time_ms: u64,

    /// Number of points returned
    pub point_count: u64,

    /// Number of series returned
    pub series_count: u64,

    /// Whether results were truncated
    pub truncated: bool,

    /// Warning messages
    #[serde(default)]
    pub warnings: Vec<String>,
}

/// Continuous query for automatic aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuousQuery {
    /// Query identifier
    pub query_id: String,

    /// Query name
    pub name: String,

    /// Source measurement
    pub source_measurement: String,

    /// Destination measurement
    pub destination_measurement: String,

    /// Query specification
    pub query: TimeSeriesQuery,

    /// Execution interval
    pub interval: String, // e.g., "1m", "5m"

    /// Whether query is enabled
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeseries_point_serialization() {
        let mut tags = TagSet::default();
        tags.source_module = "llm-observatory".to_string();
        tags.environment = "production".to_string();
        tags.model_id = Some("gpt-4".to_string());

        let fields = FieldSet::Performance(PerformanceFields {
            latency_ms: Some(150.5),
            throughput: Some(10.5),
            error_count: Some(2),
            success_count: Some(98),
            token_count: Some(1500),
            custom: HashMap::new(),
        });

        let point = TimeSeriesPoint {
            measurement: "llm_metrics".to_string(),
            timestamp: Utc::now(),
            tags,
            fields,
            metadata: None,
        };

        let json = serde_json::to_string_pretty(&point).unwrap();
        assert!(json.contains("llm_metrics"));
        assert!(json.contains("latency_ms"));
    }

    #[test]
    fn test_retention_policy_default() {
        let policy = RetentionPolicy::default();
        assert_eq!(policy.full_resolution_days, 7);
        assert_eq!(policy.max_retention_days, 365);
        assert_eq!(policy.downsample_configs.len(), 3);
    }

    #[test]
    fn test_index_config_default() {
        let config = IndexConfig::default();
        assert!(config.time_partitioning);
        assert_eq!(config.partition_interval_hours, 24);
        assert!(config.indexed_tags.contains(&"source_module".to_string()));
    }
}
