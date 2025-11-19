# LLM Analytics Hub - Data Models Documentation

## Table of Contents

1. [Overview](#overview)
2. [Analytics Event Schema](#analytics-event-schema)
3. [Metrics Aggregation Models](#metrics-aggregation-models)
4. [Time-Series Data Models](#time-series-data-models)
5. [Correlation Schemas](#correlation-schemas)
6. [Metadata Schemas](#metadata-schemas)
7. [API Response Models](#api-response-models)
8. [Examples](#examples)
9. [Best Practices](#best-practices)

---

## Overview

The LLM Analytics Hub provides a comprehensive set of data models and schemas designed for centralized monitoring and analytics across the LLM ecosystem. This documentation covers all schema definitions, data models, and their usage patterns.

### Key Features

- **Unified Event Schema**: Single schema accommodating events from all modules
- **Time-Window Aggregations**: Support for multiple time windows (1m, 5m, 1h, 1d, etc.)
- **Statistical Measures**: Complete statistical analysis (avg, min, max, p50, p95, p99)
- **Cross-Module Correlation**: Event correlation and anomaly detection across modules
- **Flexible API Models**: Pagination, error handling, streaming, and batch operations

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    LLM Analytics Hub                        │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────────┐  ┌──────────────────┐                │
│  │  Event Schemas   │  │ Metrics Models   │                │
│  │  - Telemetry     │  │ - Counter        │                │
│  │  - Security      │  │ - Gauge          │                │
│  │  - Cost          │  │ - Histogram      │                │
│  │  - Governance    │  │ - Summary        │                │
│  └──────────────────┘  └──────────────────┘                │
│                                                              │
│  ┌──────────────────┐  ┌──────────────────┐                │
│  │ Time-Series      │  │  Correlation     │                │
│  │ - Points         │  │ - Event Links    │                │
│  │ - Queries        │  │ - Anomalies      │                │
│  │ - Retention      │  │ - Root Cause     │                │
│  └──────────────────┘  └──────────────────┘                │
│                                                              │
│  ┌──────────────────┐  ┌──────────────────┐                │
│  │   Metadata       │  │  API Responses   │                │
│  │ - Assets         │  │ - Pagination     │                │
│  │ - Policies       │  │ - Errors         │                │
│  │ - Dashboards     │  │ - Streaming      │                │
│  └──────────────────┘  └──────────────────┘                │
└─────────────────────────────────────────────────────────────┘
```

---

## Analytics Event Schema

### Common Event Fields

All events share a common set of fields for consistent processing:

```rust
pub struct CommonEventFields {
    pub event_id: Uuid,              // Unique event identifier
    pub timestamp: DateTime<Utc>,    // ISO 8601 timestamp
    pub source_module: SourceModule, // Source module (Observatory, Sentinel, etc.)
    pub event_type: EventType,       // Event classification
    pub correlation_id: Option<Uuid>, // For tracing related events
    pub parent_event_id: Option<Uuid>, // Hierarchical relationships
    pub schema_version: String,      // For backward compatibility
    pub severity: Severity,          // Debug, Info, Warning, Error, Critical
    pub environment: String,         // Production, staging, etc.
    pub tags: HashMap<String, String>, // Custom tags
}
```

### Source Modules

```rust
pub enum SourceModule {
    LlmObservatory,           // Performance and telemetry
    LlmSentinel,              // Security monitoring
    LlmCostOps,               // Cost tracking
    LlmGovernanceDashboard,   // Policy and compliance
    LlmRegistry,              // Asset registry
    LlmPolicyEngine,          // Policy evaluation
    LlmAnalyticsHub,          // Self-monitoring
}
```

### Event Types

Events are categorized into distinct types:

- **Telemetry**: Performance metrics, latency, throughput
- **Security**: Threats, vulnerabilities, compliance violations
- **Cost**: Token usage, API costs, resource consumption
- **Governance**: Policy violations, audit trails
- **Lifecycle**: System events
- **Audit**: Compliance and audit events
- **Alert**: Notifications and alerts

### Module-Specific Payloads

#### Telemetry Payloads (LLM-Observatory)

```rust
pub enum TelemetryPayload {
    Latency(LatencyMetrics),
    Throughput(ThroughputMetrics),
    ErrorRate(ErrorRateMetrics),
    TokenUsage(TokenUsageMetrics),
    ModelPerformance(ModelPerformanceMetrics),
}
```

**Latency Metrics Example:**
```rust
LatencyMetrics {
    model_id: "gpt-4",
    request_id: "req-abc123",
    total_latency_ms: 1523.45,
    ttft_ms: Some(234.12),           // Time to first token
    tokens_per_second: Some(45.6),
    breakdown: Some(LatencyBreakdown {
        queue_time_ms: 150.0,
        processing_time_ms: 1200.5,
        network_time_ms: 172.95,
        other_ms: 0.0,
    }),
}
```

#### Security Payloads (LLM-Sentinel)

```rust
pub enum SecurityPayload {
    Threat(ThreatEvent),
    Vulnerability(VulnerabilityEvent),
    ComplianceViolation(ComplianceViolationEvent),
    Auth(AuthEvent),
    Privacy(PrivacyEvent),
}
```

**Threat Event Example:**
```rust
ThreatEvent {
    threat_id: "threat-789xyz",
    threat_type: ThreatType::PromptInjection,
    threat_level: ThreatLevel::High,
    source_ip: Some("192.168.1.100"),
    target_resource: "api/v1/chat/completions",
    attack_vector: "Malicious prompt",
    mitigation_status: MitigationStatus::Blocked,
    indicators_of_compromise: vec!["ioc1", "ioc2"],
}
```

#### Cost Payloads (LLM-CostOps)

```rust
pub enum CostPayload {
    TokenCost(TokenCostEvent),
    ApiCost(ApiCostEvent),
    ResourceConsumption(ResourceConsumptionEvent),
    BudgetAlert(BudgetAlertEvent),
}
```

**Token Cost Example:**
```rust
TokenCostEvent {
    model_id: "gpt-4",
    request_id: "req-cost-123",
    prompt_tokens: 500,
    completion_tokens: 300,
    total_tokens: 800,
    cost_per_prompt_token: 0.00003,
    cost_per_completion_token: 0.00006,
    total_cost_usd: 0.033,
    currency: "USD",
}
```

#### Governance Payloads (LLM-Governance-Dashboard)

```rust
pub enum GovernancePayload {
    PolicyViolation(PolicyViolationEvent),
    AuditTrail(AuditTrailEvent),
    ComplianceCheck(ComplianceCheckEvent),
    DataLineage(DataLineageEvent),
}
```

### Versioning Strategy

The schema includes a `schema_version` field for backward compatibility:

- **Major version**: Breaking changes (e.g., "2.0.0")
- **Minor version**: New optional fields (e.g., "1.1.0")
- **Patch version**: Bug fixes, documentation (e.g., "1.0.1")

Current schema version: **1.0.0**

---

## Metrics Aggregation Models

### Time Windows

Supported aggregation windows:

```rust
pub enum TimeWindow {
    OneMinute,      // 60 seconds
    FiveMinutes,    // 300 seconds
    FifteenMinutes, // 900 seconds
    OneHour,        // 3600 seconds
    SixHours,       // 21600 seconds
    OneDay,         // 86400 seconds
    OneWeek,        // 604800 seconds
    OneMonth,       // 2592000 seconds (30 days)
}
```

### Statistical Measures

Complete statistical analysis for all metrics:

```rust
pub struct StatisticalMeasures {
    pub avg: f64,      // Average/mean
    pub min: f64,      // Minimum value
    pub max: f64,      // Maximum value
    pub p50: f64,      // Median (50th percentile)
    pub p95: f64,      // 95th percentile
    pub p99: f64,      // 99th percentile
    pub stddev: Option<f64>, // Standard deviation
    pub count: u64,    // Sample count
    pub sum: f64,      // Sum of all values
}
```

### Metric Types

#### Counter Metric

Monotonically increasing value (e.g., total requests):

```rust
pub struct CounterMetric {
    pub name: String,
    pub value: u64,
    pub rate: Option<f64>,  // Rate of change (per second)
    pub tags: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}
```

#### Gauge Metric

Point-in-time value that can increase or decrease (e.g., CPU usage):

```rust
pub struct GaugeMetric {
    pub name: String,
    pub value: f64,
    pub previous_value: Option<f64>,  // For delta calculation
    pub tags: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}
```

#### Histogram Metric

Distribution of values with buckets and statistical measures:

```rust
pub struct HistogramMetric {
    pub name: String,
    pub stats: StatisticalMeasures,
    pub buckets: Vec<HistogramBucket>,
    pub tags: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}

pub struct HistogramBucket {
    pub upper_bound: f64,
    pub count: u64,
}
```

#### Summary Metric

Similar to histogram with explicit percentiles:

```rust
pub struct SummaryMetric {
    pub name: String,
    pub stats: StatisticalMeasures,
    pub percentiles: Option<HashMap<String, f64>>,
    pub tags: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}
```

### Composite Metrics

Cross-module metrics combining data from multiple sources:

```rust
pub struct CompositeMetric {
    pub metric_id: String,
    pub name: String,
    pub description: String,
    pub source_modules: Vec<String>,
    pub components: Vec<ComponentMetric>,
    pub value: f64,
    pub formula: String,  // e.g., "total_cost / total_requests"
    pub window: TimeWindow,
    pub timestamp: DateTime<Utc>,
}
```

**Example Composite Metrics:**

- **Cost Per Request**: `total_cost_usd / request_count`
- **Error-Adjusted Throughput**: `throughput * (1 - error_rate)`
- **Security-Adjusted Cost Efficiency**: `cost_efficiency * (1 - threat_score)`
- **Overall System Health Score**: Weighted average across all modules

---

## Time-Series Data Models

### Tag-Based Organization

Time-series data uses tags for filtering (low cardinality, indexed) and fields for measurements (high cardinality):

```rust
pub struct TimeSeriesPoint {
    pub measurement: String,         // e.g., "llm_latency"
    pub timestamp: DateTime<Utc>,
    pub tags: TagSet,               // Indexed for filtering
    pub fields: FieldSet,           // Measurement values
    pub metadata: Option<HashMap<String, String>>,
}
```

#### Tag Set (Low Cardinality - Indexed)

```rust
pub struct TagSet {
    pub source_module: String,      // Required
    pub environment: String,        // Required
    pub region: Option<String>,
    pub model_id: Option<String>,
    pub service: Option<String>,
    pub version: Option<String>,
    pub custom: HashMap<String, String>,  // Keep low cardinality!
}
```

#### Field Set (High Cardinality - Not Indexed)

```rust
pub enum FieldSet {
    Performance(PerformanceFields),
    Security(SecurityFields),
    Cost(CostFields),
    Governance(GovernanceFields),
    Generic(HashMap<String, FieldValue>),
}
```

### Retention Policies

Multi-tier retention with automatic downsampling:

```rust
pub struct RetentionPolicy {
    pub name: String,
    pub full_resolution_days: u32,       // Keep full resolution
    pub downsample_configs: Vec<DownsampleConfig>,
    pub max_retention_days: u32,         // Total retention
    pub shard_duration_hours: u32,
}

pub struct DownsampleConfig {
    pub after_days: u32,
    pub resolution_minutes: u32,
}
```

**Default Retention Policy:**
- Days 0-7: Full resolution (1-second intervals)
- Days 7-30: 5-minute resolution
- Days 30-90: 1-hour resolution
- Days 90-365: Daily resolution
- After 365 days: Deleted

### Indexing Strategy

Optimized for query performance:

```rust
pub struct IndexConfig {
    pub measurement: String,
    pub indexed_tags: Vec<String>,        // Ordered by cardinality
    pub shard_keys: Vec<String>,          // For distribution
    pub time_partitioning: bool,
    pub partition_interval_hours: u32,
}
```

**Best Practices:**
- Index tags with low cardinality first
- Use time-based partitioning for large datasets
- Shard by high-level dimensions (module, environment)
- Limit indexed tags to 5-7 per measurement

### Query Model

Powerful query capabilities:

```rust
pub struct TimeSeriesQuery {
    pub measurement: String,
    pub time_range: TimeRange,
    pub tag_filters: HashMap<String, String>,
    pub select_fields: Vec<String>,
    pub aggregation: Option<Aggregation>,
    pub group_by: Vec<String>,
    pub fill: Option<FillStrategy>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
```

**Fill Strategies:**
- `Null`: Leave gaps as null
- `Previous`: Forward-fill with previous value
- `Linear`: Linear interpolation
- `Zero`: Fill with zero
- `Value(i64)`: Fill with specific value

---

## Correlation Schemas

### Event Correlation

Link related events across modules:

```rust
pub struct EventCorrelation {
    pub correlation_id: CorrelationId,
    pub correlation_type: CorrelationType,
    pub events: Vec<CorrelatedEvent>,
    pub strength: f64,            // 0.0 to 1.0
    pub confidence: f64,          // 0.0 to 1.0
    pub time_window: TimeWindow,
    pub pattern: Option<CorrelationPattern>,
    pub detected_at: DateTime<Utc>,
}
```

#### Correlation Types

```rust
pub enum CorrelationType {
    CausalChain,              // Events caused by same root cause
    Temporal,                 // Events occurring simultaneously
    PatternMatch,             // Similar patterns across modules
    Anomaly,                  // Anomalous behavior correlation
    CostImpact,               // Cost impact correlation
    SecurityIncident,         // Security incident chain
    PerformanceDegradation,   // Performance degradation chain
    ComplianceCascade,        // Compliance violation cascade
}
```

### Anomaly Correlation

Detect and correlate anomalies across modules:

```rust
pub struct AnomalyCorrelation {
    pub correlation_id: CorrelationId,
    pub anomalies: Vec<AnomalyEvent>,
    pub strength: f64,
    pub root_cause: Option<RootCauseAnalysis>,
    pub impact: ImpactAssessment,
    pub detected_at: DateTime<Utc>,
}
```

#### Anomaly Types

- **Spike**: Value exceeds expected range
- **Drop**: Value below expected range
- **PatternDeviation**: Unusual pattern or trend
- **FrequencyAnomaly**: Unexpected frequency
- **DistributionShift**: Statistical distribution change

### Root Cause Analysis

Automated root cause identification:

```rust
pub struct RootCauseAnalysis {
    pub root_event_id: Uuid,
    pub confidence: f64,              // 0.0 to 1.0
    pub causal_chain: Vec<CausalLink>,
    pub contributing_factors: Vec<String>,
    pub recommendations: Vec<String>,
}

pub struct CausalLink {
    pub from_event_id: Uuid,
    pub to_event_id: Uuid,
    pub relationship: CausalRelationship,
    pub strength: f64,
    pub time_delta_ms: i64,
}
```

### Impact Assessment

Multi-dimensional impact analysis:

```rust
pub struct ImpactAssessment {
    pub severity: ImpactSeverity,
    pub affected_modules: Vec<SourceModule>,
    pub performance_impact: Option<PerformanceImpact>,
    pub cost_impact: Option<CostImpact>,
    pub security_impact: Option<SecurityImpact>,
    pub business_impact: Option<BusinessImpact>,
}
```

### Graph-Based Relationships

Model events as a directed graph:

```rust
pub struct EventGraph {
    pub graph_id: String,
    pub time_range: TimeWindow,
    pub nodes: Vec<EventNode>,
    pub edges: Vec<EventEdge>,
    pub metadata: GraphMetadata,
}

pub enum EdgeRelationship {
    Causes,
    TriggeredBy,
    RelatedTo,
    Precedes,
    Follows,
    CorrelatesWith,
    Amplifies,
    Mitigates,
}
```

---

## Metadata Schemas

### Asset Metadata (LLM-Registry)

Comprehensive asset tracking:

```rust
pub struct AssetMetadata {
    pub asset_id: String,
    pub asset_type: AssetType,
    pub name: String,
    pub description: String,
    pub version: VersionInfo,
    pub owner: OwnerInfo,
    pub tags: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: AssetStatus,
    pub metadata: AssetSpecificMetadata,
}
```

#### Asset Types

- **Model**: Language models and LLM endpoints
- **FineTunedModel**: Custom fine-tuned models
- **PromptTemplate**: Reusable prompt templates
- **Dataset**: Training and evaluation datasets
- **Endpoint**: API endpoints
- **Application**: LLM-powered applications

### Policy Definitions (LLM-Policy-Engine)

Policy configuration and rules:

```rust
pub struct PolicyDefinition {
    pub policy_id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub policy_type: PolicyType,
    pub rules: Vec<PolicyRule>,
    pub enforcement: EnforcementMode,
    pub scope: PolicyScope,
    pub status: PolicyStatus,
}
```

#### Policy Types

- **Security**: Security policies and controls
- **Compliance**: Regulatory compliance (GDPR, HIPAA, etc.)
- **CostControl**: Budget and cost management
- **Performance**: SLA and performance requirements
- **DataGovernance**: Data handling and privacy
- **Usage**: Usage quotas and limits

### Dashboard Configuration

Flexible dashboard layouts:

```rust
pub struct DashboardConfig {
    pub dashboard_id: String,
    pub name: String,
    pub layout: DashboardLayout,
    pub widgets: Vec<Widget>,
    pub refresh_interval: u32,
    pub default_time_range: String,
    pub visibility: VisibilitySettings,
}
```

#### Widget Types

- **LineChart**: Time-series line charts
- **BarChart**: Bar charts for comparisons
- **PieChart**: Distribution visualization
- **Table**: Tabular data display
- **Stat**: Single metric display
- **Heatmap**: Correlation heatmaps
- **Gauge**: Progress and threshold displays
- **Timeline**: Event timeline visualization
- **Graph**: Network/graph visualization

### User Preferences

Personalized user settings:

```rust
pub struct UserPreferences {
    pub user_id: String,
    pub display: DisplayPreferences,
    pub notifications: NotificationPreferences,
    pub default_filters: HashMap<String, String>,
    pub favorite_dashboards: Vec<String>,
    pub saved_queries: Vec<SavedQuery>,
}
```

---

## API Response Models

### Standard Response Wrapper

Consistent response format:

```rust
pub struct ApiResponse<T> {
    pub status: ResponseStatus,    // Success, Error, Partial
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub meta: ResponseMetadata,
}
```

### Pagination

Cursor and offset-based pagination:

```rust
pub struct PaginatedResponse<T> {
    pub status: ResponseStatus,
    pub data: Option<Vec<T>>,
    pub pagination: PaginationMetadata,
    pub error: Option<ApiError>,
    pub meta: ResponseMetadata,
}

pub struct PaginationMetadata {
    pub page: u32,
    pub per_page: u32,
    pub total_items: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_previous: bool,
    pub links: Option<PaginationLinks>,
}
```

### Error Handling

Comprehensive error responses:

```rust
pub struct ApiError {
    pub code: String,                    // Machine-readable
    pub message: String,                 // Human-readable
    pub status_code: u16,
    pub details: Option<ErrorDetails>,
    pub field_errors: Option<HashMap<String, Vec<String>>>,
    pub timestamp: DateTime<Utc>,
}
```

**Standard Error Codes:**
- `bad_request` (400)
- `unauthorized` (401)
- `forbidden` (403)
- `not_found` (404)
- `internal_error` (500)

### Streaming Formats

Server-Sent Events (SSE) support:

```rust
pub struct StreamEvent<T> {
    pub event_id: Uuid,
    pub event_type: StreamEventType,  // Data, Heartbeat, Error, Complete
    pub data: T,
    pub sequence: u64,
    pub timestamp: DateTime<Utc>,
}
```

### Batch Operations

Bulk operation responses:

```rust
pub struct BatchResponse<T> {
    pub batch_id: Uuid,
    pub total_items: usize,
    pub success_count: usize,
    pub failure_count: usize,
    pub results: Vec<BatchItemResult<T>>,
    pub status: BatchStatus,  // AllSuccess, PartialSuccess, AllFailed
}
```

---

## Examples

### Running Examples

```bash
# Build the project
cargo build

# Run event examples
cargo run --example event_examples

# Run metrics examples
cargo run --example metrics_examples
```

### Example: Creating a Telemetry Event

```rust
use llm_analytics_hub::*;
use chrono::Utc;
use uuid::Uuid;

let event = AnalyticsEvent {
    common: CommonEventFields {
        event_id: Uuid::new_v4(),
        timestamp: Utc::now(),
        source_module: SourceModule::LlmObservatory,
        event_type: EventType::Telemetry,
        correlation_id: None,
        parent_event_id: None,
        schema_version: "1.0.0".to_string(),
        severity: Severity::Info,
        environment: "production".to_string(),
        tags: HashMap::new(),
    },
    payload: EventPayload::Telemetry(
        TelemetryPayload::Latency(LatencyMetrics {
            model_id: "gpt-4".to_string(),
            request_id: "req-123".to_string(),
            total_latency_ms: 1523.45,
            ttft_ms: Some(234.12),
            tokens_per_second: Some(45.6),
            breakdown: None,
        })
    ),
};

// Serialize to JSON
let json = serde_json::to_string_pretty(&event)?;
```

### Example: Querying Time-Series Data

```rust
use llm_analytics_hub::models::timeseries::*;

let query = TimeSeriesQuery {
    measurement: "llm_latency".to_string(),
    time_range: TimeRange {
        start: Utc::now() - Duration::hours(24),
        end: Utc::now(),
    },
    tag_filters: {
        let mut filters = HashMap::new();
        filters.insert("model_id".to_string(), "gpt-4".to_string());
        filters.insert("environment".to_string(), "production".to_string());
        filters
    },
    select_fields: vec!["latency_ms".to_string()],
    aggregation: Some(Aggregation {
        function: AggregationFunction::Mean,
        window: "5m".to_string(),
        fields: vec!["latency_ms".to_string()],
    }),
    group_by: vec!["region".to_string()],
    fill: Some(FillStrategy::Linear),
    limit: Some(1000),
    offset: None,
};
```

---

## Best Practices

### Event Schema

1. **Always include correlation_id** for distributed tracing
2. **Use appropriate severity levels** to enable effective filtering
3. **Include relevant tags** for dimensional analysis
4. **Maintain schema versioning** for backward compatibility

### Metrics

1. **Choose the right metric type**:
   - Counter for cumulative values
   - Gauge for point-in-time values
   - Histogram for distributions
   - Summary for percentiles

2. **Use appropriate time windows**:
   - Real-time monitoring: 1-5 minutes
   - Recent trends: 1 hour
   - Daily reports: 1 day
   - Historical analysis: 1 week or more

3. **Tag wisely**:
   - Keep tag cardinality low (< 1000 unique values)
   - Use tags for filtering, fields for measurements
   - Common tags: environment, region, model_id

### Time-Series

1. **Index strategy**:
   - Index frequently queried tags
   - Order indexed tags by cardinality (low to high)
   - Limit to 5-7 indexed tags per measurement

2. **Retention policy**:
   - Balance storage costs vs. query needs
   - Use downsampling for historical data
   - Set appropriate shard durations

3. **Query optimization**:
   - Use tag filters to reduce data scanned
   - Select only needed fields
   - Limit result sets appropriately

### Correlation

1. **Set appropriate thresholds**:
   - Correlation strength: > 0.7 for meaningful correlations
   - Anomaly score: > 0.8 for critical anomalies
   - Confidence: > 0.85 for automated actions

2. **Time windows**:
   - Short windows (5-15 min) for real-time correlation
   - Longer windows (1-24 hours) for pattern analysis

3. **Impact assessment**:
   - Always include impact metrics for critical correlations
   - Prioritize by business impact severity

### API Design

1. **Pagination**:
   - Default: 50 items per page
   - Maximum: 1000 items per page
   - Always include pagination links

2. **Error handling**:
   - Use appropriate HTTP status codes
   - Provide actionable error messages
   - Include suggestions for resolution

3. **Performance**:
   - Cache frequently accessed data
   - Use streaming for large datasets
   - Implement rate limiting

---

## Version History

- **1.0.0** (2024): Initial release
  - Complete event schema for all modules
  - Metrics aggregation models
  - Time-series data models
  - Correlation and anomaly detection
  - Comprehensive API models

---

## License

Apache License 2.0 - See LICENSE file for details
