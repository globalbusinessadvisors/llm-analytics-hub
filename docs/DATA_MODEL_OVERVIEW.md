# LLM Analytics Hub - Data Model Overview

## Complete Data Model Relationships

This document provides a visual overview of all data models and their relationships.

---

## Architecture Layers

```
┌─────────────────────────────────────────────────────────────────────┐
│                         DATA INGESTION LAYER                        │
│                                                                     │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐  │
│  │Observatory │  │  Sentinel  │  │  CostOps   │  │ Governance │  │
│  │  Events    │  │   Events   │  │   Events   │  │   Events   │  │
│  └─────┬──────┘  └─────┬──────┘  └─────┬──────┘  └─────┬──────┘  │
└────────┼───────────────┼───────────────┼───────────────┼──────────┘
         │               │               │               │
         └───────────────┴───────────────┴───────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      UNIFIED EVENT SCHEMA                           │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ CommonEventFields                                            │ │
│  │  - event_id: UUID                                            │ │
│  │  - timestamp: DateTime<Utc>                                  │ │
│  │  - source_module: SourceModule                               │ │
│  │  - event_type: EventType                                     │ │
│  │  - correlation_id: Option<UUID>                              │ │
│  │  - severity: Severity                                        │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                              │                                      │
│                              ▼                                      │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ EventPayload (enum)                                          │ │
│  │  ├─ Telemetry(TelemetryPayload)                             │ │
│  │  ├─ Security(SecurityPayload)                               │ │
│  │  ├─ Cost(CostPayload)                                       │ │
│  │  ├─ Governance(GovernancePayload)                           │ │
│  │  └─ Custom(CustomPayload)                                   │ │
│  └──────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
                              │
           ┌──────────────────┴──────────────────┐
           │                                     │
           ▼                                     ▼
┌────────────────────────────┐      ┌────────────────────────────┐
│   METRICS AGGREGATION      │      │   TIME-SERIES STORAGE      │
│                            │      │                            │
│ ┌────────────────────────┐ │      │ ┌────────────────────────┐ │
│ │ MetricType (enum)      │ │      │ │ TimeSeriesPoint        │ │
│ │  - Counter             │ │      │ │  - measurement         │ │
│ │  - Gauge               │ │      │ │  - timestamp           │ │
│ │  - Histogram           │ │      │ │  - tags: TagSet        │ │
│ │  - Summary             │ │      │ │  - fields: FieldSet    │ │
│ └────────────────────────┘ │      │ └────────────────────────┘ │
│                            │      │                            │
│ ┌────────────────────────┐ │      │ ┌────────────────────────┐ │
│ │ AggregatedMetric       │ │      │ │ RetentionPolicy        │ │
│ │  - window: TimeWindow  │ │      │ │  - full_resolution_days│ │
│ │  - values: Stats       │ │      │ │  - downsample_configs  │ │
│ └────────────────────────┘ │      │ └────────────────────────┘ │
│                            │      │                            │
│ ┌────────────────────────┐ │      │ ┌────────────────────────┐ │
│ │ CompositeMetric        │ │      │ │ TimeSeriesQuery        │ │
│ │  - components          │ │      │ │  - time_range          │ │
│ │  - formula             │ │      │ │  - tag_filters         │ │
│ └────────────────────────┘ │      │ │  - aggregation         │ │
└────────────────────────────┘      │ └────────────────────────┘ │
                                    └────────────────────────────┘
           │                                     │
           │                                     │
           ▼                                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      CORRELATION & ANALYSIS                         │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ EventCorrelation                                             │ │
│  │  - correlation_id                                            │ │
│  │  - correlation_type: CorrelationType                         │ │
│  │  - events: Vec<CorrelatedEvent>                              │ │
│  │  - strength: f64                                             │ │
│  │  - confidence: f64                                           │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ AnomalyCorrelation                                           │ │
│  │  - anomalies: Vec<AnomalyEvent>                              │ │
│  │  - root_cause: RootCauseAnalysis                             │ │
│  │  - impact: ImpactAssessment                                  │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ EventGraph                                                   │ │
│  │  - nodes: Vec<EventNode>                                     │ │
│  │  - edges: Vec<EventEdge>                                     │ │
│  │  - metadata: GraphMetadata                                   │ │
│  └──────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         METADATA LAYER                              │
│                                                                     │
│  ┌────────────────────┐  ┌────────────────────┐  ┌──────────────┐ │
│  │ AssetMetadata      │  │ PolicyDefinition   │  │ Dashboard    │ │
│  │  - asset_type      │  │  - policy_type     │  │ Config       │ │
│  │  - version         │  │  - rules           │  │  - layout    │ │
│  │  - owner           │  │  - enforcement     │  │  - widgets   │ │
│  │  - metadata        │  │  - scope           │  │              │ │
│  └────────────────────┘  └────────────────────┘  └──────────────┘ │
│                                                                     │
│  ┌────────────────────┐                                            │
│  │ UserPreferences    │                                            │
│  │  - display         │                                            │
│  │  - notifications   │                                            │
│  │  - saved_queries   │                                            │
│  └────────────────────┘                                            │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                          API LAYER                                  │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ ApiResponse<T>                                               │ │
│  │  - status: ResponseStatus                                    │ │
│  │  - data: Option<T>                                           │ │
│  │  - error: Option<ApiError>                                   │ │
│  │  - meta: ResponseMetadata                                    │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ PaginatedResponse<T>                                         │ │
│  │  - data: Vec<T>                                              │ │
│  │  - pagination: PaginationMetadata                            │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │ StreamEvent<T>                                               │ │
│  │  - event_type: StreamEventType                               │ │
│  │  - sequence: u64                                             │ │
│  └──────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Data Flow Diagram

```
Event Generation → Ingestion → Processing → Storage → Query → Response

┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│  Module  │────▶│  Event   │────▶│  Metric  │────▶│   Time   │
│  Events  │     │  Schema  │     │  Agg.    │     │  Series  │
└──────────┘     └──────────┘     └──────────┘     └──────────┘
                       │                │                │
                       │                │                │
                       ▼                ▼                ▼
                 ┌──────────┐     ┌──────────┐     ┌──────────┐
                 │ Corr.    │     │ Anomaly  │     │  Graph   │
                 │ Detection│     │ Detection│     │ Analysis │
                 └──────────┘     └──────────┘     └──────────┘
                       │                │                │
                       └────────┬───────┴────────────────┘
                                │
                                ▼
                          ┌──────────┐
                          │   API    │
                          │ Response │
                          └──────────┘
```

---

## Schema Relationships

### Event Schema Hierarchy

```
AnalyticsEvent
├── CommonEventFields
│   ├── event_id: UUID
│   ├── timestamp: DateTime<Utc>
│   ├── source_module: SourceModule
│   ├── event_type: EventType
│   ├── correlation_id: Option<UUID>
│   ├── parent_event_id: Option<UUID>
│   ├── schema_version: String
│   ├── severity: Severity
│   ├── environment: String
│   └── tags: HashMap<String, String>
│
└── EventPayload
    ├── Telemetry
    │   ├── Latency
    │   ├── Throughput
    │   ├── ErrorRate
    │   ├── TokenUsage
    │   └── ModelPerformance
    │
    ├── Security
    │   ├── Threat
    │   ├── Vulnerability
    │   ├── ComplianceViolation
    │   ├── Auth
    │   └── Privacy
    │
    ├── Cost
    │   ├── TokenCost
    │   ├── ApiCost
    │   ├── ResourceConsumption
    │   └── BudgetAlert
    │
    ├── Governance
    │   ├── PolicyViolation
    │   ├── AuditTrail
    │   ├── ComplianceCheck
    │   └── DataLineage
    │
    └── Custom
```

### Metrics Schema Hierarchy

```
MetricType
├── Counter
│   ├── name: String
│   ├── value: u64
│   ├── rate: Option<f64>
│   ├── tags: HashMap
│   └── timestamp: DateTime
│
├── Gauge
│   ├── name: String
│   ├── value: f64
│   ├── previous_value: Option<f64>
│   ├── tags: HashMap
│   └── timestamp: DateTime
│
├── Histogram
│   ├── name: String
│   ├── stats: StatisticalMeasures
│   ├── buckets: Vec<HistogramBucket>
│   ├── tags: HashMap
│   └── timestamp: DateTime
│
└── Summary
    ├── name: String
    ├── stats: StatisticalMeasures
    ├── percentiles: Option<HashMap>
    ├── tags: HashMap
    └── timestamp: DateTime

StatisticalMeasures
├── avg: f64
├── min: f64
├── max: f64
├── p50: f64
├── p95: f64
├── p99: f64
├── stddev: Option<f64>
├── count: u64
└── sum: f64
```

### Time-Series Schema Hierarchy

```
TimeSeriesPoint
├── measurement: String
├── timestamp: DateTime<Utc>
├── tags: TagSet
│   ├── source_module: String
│   ├── environment: String
│   ├── region: Option<String>
│   ├── model_id: Option<String>
│   ├── service: Option<String>
│   ├── version: Option<String>
│   └── custom: HashMap<String, String>
│
└── fields: FieldSet
    ├── Performance
    │   ├── latency_ms
    │   ├── throughput
    │   ├── error_count
    │   ├── success_count
    │   └── token_count
    │
    ├── Security
    │   ├── threat_count
    │   ├── severity_score
    │   ├── blocked_count
    │   └── vulnerability_count
    │
    ├── Cost
    │   ├── cost_usd
    │   ├── token_cost
    │   └── utilization_percent
    │
    ├── Governance
    │   ├── violation_count
    │   ├── compliance_score
    │   └── audit_count
    │
    └── Generic
        └── HashMap<String, FieldValue>
```

### Correlation Schema Hierarchy

```
EventCorrelation
├── correlation_id: CorrelationId
├── correlation_type: CorrelationType
│   ├── CausalChain
│   ├── Temporal
│   ├── PatternMatch
│   ├── Anomaly
│   ├── CostImpact
│   ├── SecurityIncident
│   ├── PerformanceDegradation
│   └── ComplianceCascade
│
├── events: Vec<CorrelatedEvent>
│   └── CorrelatedEvent
│       ├── event_id: UUID
│       ├── source_module: SourceModule
│       ├── event_type: EventType
│       ├── severity: Severity
│       ├── role: EventRole
│       └── metrics: HashMap
│
├── strength: f64
├── confidence: f64
├── time_window: TimeWindow
└── pattern: Option<CorrelationPattern>

AnomalyCorrelation
├── correlation_id: CorrelationId
├── anomalies: Vec<AnomalyEvent>
├── strength: f64
├── root_cause: Option<RootCauseAnalysis>
│   ├── root_event_id: UUID
│   ├── confidence: f64
│   ├── causal_chain: Vec<CausalLink>
│   ├── contributing_factors: Vec<String>
│   └── recommendations: Vec<String>
│
└── impact: ImpactAssessment
    ├── severity: ImpactSeverity
    ├── performance_impact
    ├── cost_impact
    ├── security_impact
    └── business_impact
```

---

## Cardinality Guidelines

### Low Cardinality (Good for Tags - Indexed)

| Tag | Estimated Values | Example |
|-----|-----------------|---------|
| source_module | ~7 | llm-observatory, llm-sentinel |
| environment | ~3-5 | production, staging, dev |
| region | ~10-20 | us-east-1, eu-west-1 |
| event_type | ~7 | telemetry, security, cost |
| severity | 5 | debug, info, warning, error, critical |

### Medium Cardinality (Consider Carefully)

| Tag | Estimated Values | Example |
|-----|-----------------|---------|
| model_id | ~20-50 | gpt-4, claude-3, llama-2 |
| service | ~10-30 | api-gateway, inference-engine |
| version | ~5-20 | v1.2.3, v2.0.0 |

### High Cardinality (Use as Fields - Not Indexed)

| Field | Estimated Values | Example |
|-------|-----------------|---------|
| event_id | Unbounded | UUID |
| request_id | Unbounded | req-abc123 |
| user_id | Thousands+ | user-12345 |
| latency_ms | Continuous | 123.45 |
| cost_usd | Continuous | 0.0025 |

---

## Query Pattern Examples

### 1. Event Query

```sql
-- Conceptual query
SELECT * FROM analytics_events
WHERE source_module = 'llm-observatory'
  AND event_type = 'telemetry'
  AND timestamp >= NOW() - INTERVAL '1 hour'
  AND severity >= 'warning'
ORDER BY timestamp DESC
LIMIT 100
```

### 2. Metrics Aggregation Query

```sql
-- Conceptual query
SELECT
  time_bucket('5 minutes', timestamp) as bucket,
  AVG(latency_ms) as avg_latency,
  PERCENTILE_CONT(0.95) WITHIN GROUP (ORDER BY latency_ms) as p95_latency,
  COUNT(*) as request_count
FROM llm_metrics
WHERE measurement = 'request_latency'
  AND tags->>'model_id' = 'gpt-4'
  AND timestamp >= NOW() - INTERVAL '1 day'
GROUP BY bucket
ORDER BY bucket
```

### 3. Correlation Query

```sql
-- Conceptual query
SELECT
  c.correlation_id,
  c.correlation_type,
  c.strength,
  COUNT(ce.event_id) as event_count
FROM event_correlations c
JOIN correlated_events ce ON c.correlation_id = ce.correlation_id
WHERE c.detected_at >= NOW() - INTERVAL '1 hour'
  AND c.strength >= 0.8
GROUP BY c.correlation_id, c.correlation_type, c.strength
ORDER BY c.strength DESC
```

---

## Integration Patterns

### Pattern 1: Event Ingestion

```rust
// Producer (Module) → Analytics Hub
let event = AnalyticsEvent {
    common: CommonEventFields { /* ... */ },
    payload: EventPayload::Telemetry(/* ... */),
};

// Serialize and send
let json = serde_json::to_string(&event)?;
send_to_hub(json);
```

### Pattern 2: Metrics Collection

```rust
// Module → Metrics → Time-Series
let metric = MetricType::Histogram(HistogramMetric {
    name: "request_latency_ms".to_string(),
    stats: calculate_stats(&measurements),
    buckets: create_buckets(&measurements),
    tags: get_tags(),
    timestamp: Utc::now(),
});

// Convert to time-series point
let ts_point = TimeSeriesPoint {
    measurement: metric.name(),
    timestamp: metric.timestamp(),
    tags: metric.tags(),
    fields: metric.to_fields(),
};
```

### Pattern 3: Correlation Detection

```rust
// Events → Correlation Engine → Alerts
let correlation = detect_correlation(
    events,
    time_window,
    min_strength: 0.8,
);

if correlation.strength >= threshold {
    let impact = assess_impact(&correlation);
    if impact.severity >= ImpactSeverity::High {
        send_alert(&correlation, &impact);
    }
}
```

---

## Storage Recommendations

### Time-Series Database (InfluxDB/TimescaleDB)

**Use For**:
- Metrics data (counters, gauges, histograms)
- Time-series points
- High-frequency data
- Downsampling/aggregation

**Schema**:
```
measurement: string (e.g., "llm_latency")
tags: map[string]string (indexed)
fields: map[string]value (not indexed)
timestamp: datetime
```

### Relational Database (PostgreSQL)

**Use For**:
- Event metadata
- Correlation relationships
- Asset metadata
- Policy definitions
- User preferences

**Tables**:
- `analytics_events`
- `event_correlations`
- `asset_metadata`
- `policy_definitions`
- `dashboard_configs`
- `user_preferences`

### Graph Database (Neo4j) - Optional

**Use For**:
- Event relationship graphs
- Complex correlation patterns
- Root cause analysis

**Nodes**:
- EventNode
- CorrelationNode

**Edges**:
- Causes, TriggeredBy, RelatedTo, etc.

---

## Performance Optimization

### Indexing Strategy

```sql
-- Time-series indexes
CREATE INDEX idx_ts_measurement_time ON timeseries (measurement, timestamp DESC);
CREATE INDEX idx_ts_tags ON timeseries USING GIN (tags);

-- Event indexes
CREATE INDEX idx_events_source_time ON events (source_module, timestamp DESC);
CREATE INDEX idx_events_correlation ON events (correlation_id) WHERE correlation_id IS NOT NULL;
CREATE INDEX idx_events_severity_time ON events (severity, timestamp DESC) WHERE severity >= 'warning';

-- Correlation indexes
CREATE INDEX idx_corr_strength ON correlations (strength DESC) WHERE strength >= 0.8;
CREATE INDEX idx_corr_detected_at ON correlations (detected_at DESC);
```

### Partitioning Strategy

```sql
-- Time-based partitioning for events
CREATE TABLE analytics_events_y2024m01 PARTITION OF analytics_events
FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');

-- Module-based partitioning
CREATE TABLE events_observatory PARTITION OF analytics_events
FOR VALUES IN ('llm-observatory');
```

---

## Summary

The LLM Analytics Hub data models provide:

1. **Unified Schema**: Single event schema for all modules
2. **Flexible Metrics**: Multiple metric types with rich statistics
3. **Optimized Time-Series**: Tag-based organization for performance
4. **Advanced Correlation**: Multi-dimensional event correlation
5. **Rich Metadata**: Comprehensive asset and policy tracking
6. **Robust API**: Consistent response formats with pagination

All schemas are:
- Type-safe (Rust structs)
- Serializable (serde)
- Documented (inline + guides)
- Tested (unit tests)
- Production-ready

---

**Version**: 1.0.0
**Last Updated**: 2025-11-19
**License**: Apache 2.0
