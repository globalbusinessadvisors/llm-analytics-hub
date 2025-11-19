# Schema Reference Guide

## Quick Reference

This document provides a quick reference for all schemas and data models in the LLM Analytics Hub.

## Table of Contents

- [Event Schema Summary](#event-schema-summary)
- [Metrics Types](#metrics-types)
- [Time-Series Fields](#time-series-fields)
- [Correlation Types](#correlation-types)
- [API Response Codes](#api-response-codes)
- [JSON Examples](#json-examples)

---

## Event Schema Summary

### Event Severity Levels

| Level | Value | Description | Use Case |
|-------|-------|-------------|----------|
| Debug | 0 | Detailed diagnostic info | Development, troubleshooting |
| Info | 1 | Normal operation events | General monitoring |
| Warning | 2 | Potential issues | Attention needed |
| Error | 3 | Error conditions | Immediate attention |
| Critical | 4 | System-critical failures | Emergency response |

### Source Modules

| Module | Prefix | Primary Function |
|--------|--------|-----------------|
| llm-observatory | `obs-` | Performance & telemetry monitoring |
| llm-sentinel | `sec-` | Security threat detection |
| llm-costops | `cost-` | Cost tracking & optimization |
| llm-governance-dashboard | `gov-` | Policy & compliance |
| llm-registry | `reg-` | Asset management |
| llm-policy-engine | `pol-` | Policy evaluation |
| llm-analytics-hub | `hub-` | Analytics aggregation |

### Event Type Matrix

| Event Type | Observatory | Sentinel | CostOps | Governance |
|-----------|-------------|----------|---------|------------|
| Telemetry | ✓ | - | - | - |
| Security | - | ✓ | - | - |
| Cost | - | - | ✓ | - |
| Governance | - | - | - | ✓ |
| Audit | ✓ | ✓ | ✓ | ✓ |
| Alert | ✓ | ✓ | ✓ | ✓ |

---

## Metrics Types

### Metric Type Selection Guide

| Metric Type | When to Use | Example |
|------------|-------------|---------|
| **Counter** | Cumulative, always increasing | `total_requests`, `bytes_sent` |
| **Gauge** | Point-in-time, can go up/down | `cpu_usage`, `active_connections` |
| **Histogram** | Distribution with buckets | `request_duration`, `response_size` |
| **Summary** | Distribution with percentiles | `latency_percentiles` |

### Time Windows

| Window | Duration | Seconds | Use Case |
|--------|----------|---------|----------|
| 1m | 1 minute | 60 | Real-time monitoring |
| 5m | 5 minutes | 300 | Recent trends |
| 15m | 15 minutes | 900 | Short-term analysis |
| 1h | 1 hour | 3,600 | Hourly reports |
| 6h | 6 hours | 21,600 | Shift analysis |
| 1d | 1 day | 86,400 | Daily reports |
| 1w | 1 week | 604,800 | Weekly trends |
| 1M | 1 month | 2,592,000 | Monthly analysis |

### Statistical Measures

| Measure | Field | Description |
|---------|-------|-------------|
| Average | `avg` | Mean value |
| Minimum | `min` | Lowest value |
| Maximum | `max` | Highest value |
| Median | `p50` | 50th percentile |
| 95th Percentile | `p95` | 95% of values below this |
| 99th Percentile | `p99` | 99% of values below this |
| Std Deviation | `stddev` | Standard deviation |
| Count | `count` | Number of samples |
| Sum | `sum` | Total sum |

---

## Time-Series Fields

### Standard Tags (Low Cardinality - Indexed)

| Tag | Required | Example Values | Cardinality |
|-----|----------|----------------|-------------|
| `source_module` | Yes | `llm-observatory`, `llm-sentinel` | ~7 |
| `environment` | Yes | `production`, `staging`, `dev` | ~3-5 |
| `region` | No | `us-east-1`, `eu-west-1` | ~10-20 |
| `model_id` | No | `gpt-4`, `claude-3` | ~20-50 |
| `service` | No | `api-gateway`, `inference` | ~10-30 |
| `version` | No | `v1.2.3`, `latest` | ~5-20 |

### Performance Fields

| Field | Type | Unit | Description |
|-------|------|------|-------------|
| `latency_ms` | f64 | milliseconds | Request latency |
| `throughput` | f64 | req/sec | Requests per second |
| `error_count` | u64 | count | Number of errors |
| `success_count` | u64 | count | Successful requests |
| `token_count` | u64 | tokens | Tokens processed |

### Security Fields

| Field | Type | Description |
|-------|------|-------------|
| `threat_count` | u64 | Number of threats detected |
| `severity_score` | f64 | Threat severity (0.0-10.0) |
| `blocked_count` | u64 | Blocked requests |
| `vulnerability_count` | u64 | Vulnerabilities found |

### Cost Fields

| Field | Type | Unit | Description |
|-------|------|------|-------------|
| `cost_usd` | f64 | USD | Total cost |
| `token_cost` | f64 | USD | Token-related costs |
| `utilization_percent` | f64 | % | Resource utilization |

---

## Correlation Types

### Correlation Type Matrix

| Type | Description | Typical Strength | Use Case |
|------|-------------|-----------------|----------|
| `causal_chain` | Direct cause-effect | 0.8-1.0 | Root cause analysis |
| `temporal` | Time-based correlation | 0.6-0.9 | Incident investigation |
| `pattern_match` | Similar patterns | 0.7-0.9 | Anomaly detection |
| `anomaly` | Anomalous behavior | 0.7-0.95 | Alert correlation |
| `cost_impact` | Cost relationship | 0.6-0.85 | Cost optimization |
| `security_incident` | Security chain | 0.75-0.95 | Security response |
| `performance_degradation` | Performance chain | 0.7-0.9 | Performance tuning |
| `compliance_cascade` | Compliance violations | 0.8-0.95 | Compliance monitoring |

### Anomaly Types

| Type | Detection Method | Example |
|------|-----------------|---------|
| `spike` | Value > baseline + threshold | CPU suddenly jumps to 90% |
| `drop` | Value < baseline - threshold | Request rate drops to 10% |
| `pattern_deviation` | Pattern analysis | Different daily pattern |
| `frequency_anomaly` | Frequency analysis | Too many/few events |
| `distribution_shift` | Statistical tests | Changed data distribution |

### Impact Severity

| Level | Score Range | Response Time | Escalation |
|-------|-------------|---------------|------------|
| Negligible | 0.0-0.2 | None | Log only |
| Low | 0.2-0.4 | 24 hours | Standard |
| Medium | 0.4-0.6 | 4 hours | Team lead |
| High | 0.6-0.8 | 1 hour | Manager |
| Critical | 0.8-1.0 | Immediate | Executive |

---

## API Response Codes

### HTTP Status Codes

| Code | Status | Meaning | Response Includes |
|------|--------|---------|-------------------|
| 200 | OK | Success | Data |
| 201 | Created | Resource created | Data + Location |
| 400 | Bad Request | Invalid input | Error details |
| 401 | Unauthorized | Auth required | WWW-Authenticate |
| 403 | Forbidden | Permission denied | Error |
| 404 | Not Found | Resource not found | Suggestions |
| 429 | Too Many Requests | Rate limited | Retry-After |
| 500 | Internal Error | Server error | Error + trace |
| 503 | Service Unavailable | Temporary outage | Retry-After |

### Error Codes

| Code | HTTP | Description | Retry? |
|------|------|-------------|--------|
| `bad_request` | 400 | Invalid request format | No |
| `validation_error` | 400 | Field validation failed | No |
| `unauthorized` | 401 | Authentication required | Yes (with auth) |
| `forbidden` | 403 | Insufficient permissions | No |
| `not_found` | 404 | Resource doesn't exist | No |
| `conflict` | 409 | Resource conflict | Maybe |
| `rate_limited` | 429 | Too many requests | Yes (after delay) |
| `internal_error` | 500 | Server-side error | Yes |
| `service_unavailable` | 503 | Temporary outage | Yes |

### Query Status Codes

| Status | Meaning | Data Present | Action |
|--------|---------|--------------|--------|
| `success` | Complete success | Yes | Use data |
| `partial_success` | Some data unavailable | Partial | Check warnings |
| `failed` | Query failed | No | Retry or modify |
| `timeout` | Query timed out | No | Reduce scope |

---

## JSON Examples

### Minimal Telemetry Event

```json
{
  "event_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-01-15T10:30:00Z",
  "source_module": "llm-observatory",
  "event_type": "telemetry",
  "schema_version": "1.0.0",
  "severity": "info",
  "environment": "production",
  "tags": {},
  "payload_type": "telemetry",
  "data": {
    "telemetry_type": "latency",
    "model_id": "gpt-4",
    "request_id": "req-123",
    "total_latency_ms": 1523.45
  }
}
```

### Security Threat Event

```json
{
  "event_id": "650e8400-e29b-41d4-a716-446655440001",
  "timestamp": "2024-01-15T10:35:00Z",
  "source_module": "llm-sentinel",
  "event_type": "security",
  "severity": "critical",
  "environment": "production",
  "tags": {
    "threat_category": "injection"
  },
  "payload_type": "security",
  "data": {
    "security_type": "threat",
    "threat_id": "threat-789",
    "threat_type": "prompt_injection",
    "threat_level": "high",
    "mitigation_status": "blocked"
  }
}
```

### Cost Budget Alert

```json
{
  "event_id": "750e8400-e29b-41d4-a716-446655440002",
  "timestamp": "2024-01-15T10:40:00Z",
  "source_module": "llm-costops",
  "event_type": "alert",
  "severity": "warning",
  "environment": "production",
  "payload_type": "cost",
  "data": {
    "cost_type": "budget_alert",
    "budget_id": "monthly-2024-01",
    "budget_limit_usd": 10000.0,
    "current_spend_usd": 8500.0,
    "threshold_percent": 85.0,
    "alert_type": "warning"
  }
}
```

### Counter Metric

```json
{
  "metric_type": "counter",
  "name": "llm_requests_total",
  "value": 125834,
  "rate": 45.5,
  "tags": {
    "model": "gpt-4",
    "environment": "production"
  },
  "timestamp": "2024-01-15T10:45:00Z"
}
```

### Histogram Metric

```json
{
  "metric_type": "histogram",
  "name": "request_latency_ms",
  "stats": {
    "avg": 450.5,
    "min": 50.0,
    "max": 4800.0,
    "p50": 380.0,
    "p95": 1200.0,
    "p99": 2500.0,
    "count": 7000,
    "sum": 3153500.0
  },
  "buckets": [
    {"upper_bound": 100.0, "count": 1500},
    {"upper_bound": 500.0, "count": 3200},
    {"upper_bound": 1000.0, "count": 1800},
    {"upper_bound": 5000.0, "count": 500}
  ],
  "tags": {"endpoint": "/api/v1/chat"},
  "timestamp": "2024-01-15T10:50:00Z"
}
```

### Time-Series Point

```json
{
  "measurement": "llm_performance",
  "timestamp": "2024-01-15T10:55:00Z",
  "tags": {
    "source_module": "llm-observatory",
    "environment": "production",
    "model_id": "gpt-4",
    "region": "us-east-1"
  },
  "fields": {
    "latency_ms": 456.8,
    "throughput": 125.5,
    "error_count": 3,
    "success_count": 2497,
    "token_count": 45000
  }
}
```

### API Success Response

```json
{
  "status": "success",
  "data": {
    "metrics": ["metric-1", "metric-2", "metric-3"]
  },
  "meta": {
    "request_id": "850e8400-e29b-41d4-a716-446655440003",
    "timestamp": "2024-01-15T11:00:00Z",
    "api_version": "1.0.0",
    "response_time_ms": 45
  }
}
```

### API Error Response

```json
{
  "status": "error",
  "error": {
    "code": "not_found",
    "message": "Metric 'invalid-metric' not found",
    "status_code": 404,
    "details": {
      "suggestions": [
        "Check the metric name spelling",
        "List available metrics with GET /metrics"
      ],
      "documentation_url": "https://docs.example.com/metrics"
    },
    "timestamp": "2024-01-15T11:05:00Z"
  },
  "meta": {
    "request_id": "950e8400-e29b-41d4-a716-446655440004",
    "timestamp": "2024-01-15T11:05:00Z",
    "api_version": "1.0.0"
  }
}
```

### Paginated Response

```json
{
  "status": "success",
  "data": ["item1", "item2", "item3"],
  "pagination": {
    "page": 2,
    "per_page": 3,
    "total_items": 15,
    "total_pages": 5,
    "has_next": true,
    "has_previous": true,
    "links": {
      "first": "https://api.example.com/metrics?page=1",
      "last": "https://api.example.com/metrics?page=5",
      "next": "https://api.example.com/metrics?page=3",
      "prev": "https://api.example.com/metrics?page=1",
      "self_link": "https://api.example.com/metrics?page=2"
    }
  },
  "meta": {
    "request_id": "a50e8400-e29b-41d4-a716-446655440005",
    "timestamp": "2024-01-15T11:10:00Z",
    "api_version": "1.0.0"
  }
}
```

---

## Field Naming Conventions

### General Rules

1. Use **snake_case** for all field names
2. Be descriptive but concise
3. Use standard units in field names (e.g., `_ms`, `_usd`, `_percent`)
4. Prefer explicit names over abbreviations
5. Use consistent terminology across modules

### Common Suffixes

| Suffix | Meaning | Example |
|--------|---------|---------|
| `_id` | Identifier | `event_id`, `model_id` |
| `_at` | Timestamp | `created_at`, `detected_at` |
| `_ms` | Milliseconds | `latency_ms`, `ttft_ms` |
| `_usd` | US Dollars | `cost_usd`, `budget_usd` |
| `_percent` | Percentage | `error_rate_percent`, `utilization_percent` |
| `_count` | Counter | `error_count`, `request_count` |
| `_score` | Numeric score | `severity_score`, `quality_score` |
| `_rate` | Rate (per time) | `error_rate`, `throughput_rate` |

### Reserved Field Names

Do not use these names for custom fields:

- `id`, `_id`
- `timestamp`, `time`
- `type`, `kind`
- `status`, `state`
- `version`
- `metadata`, `meta`
- `tags`, `labels`
- `data`, `payload`

---

## Data Type Reference

### Primitive Types

| Type | Rust | JSON | Range/Format |
|------|------|------|--------------|
| String | `String` | `"text"` | UTF-8 |
| Integer | `i64` | `123` | -2^63 to 2^63-1 |
| Unsigned | `u64` | `123` | 0 to 2^64-1 |
| Float | `f64` | `123.45` | IEEE 754 double |
| Boolean | `bool` | `true/false` | true or false |
| Timestamp | `DateTime<Utc>` | `"2024-01-15T10:30:00Z"` | ISO 8601 |
| UUID | `Uuid` | `"550e8400-..."` | RFC 4122 |

### Complex Types

| Type | Format | Example |
|------|--------|---------|
| HashMap | `{"key": "value"}` | Tag sets, metadata |
| Array | `[item1, item2]` | Lists of values |
| Enum | `"variant_name"` | Status, types |
| Optional | `null` or value | Optional fields |

---

## Version Compatibility

### Schema Version Format

`MAJOR.MINOR.PATCH` (Semantic Versioning)

- **MAJOR**: Breaking changes (incompatible API changes)
- **MINOR**: New features (backward-compatible)
- **PATCH**: Bug fixes (backward-compatible)

### Current Version: 1.0.0

### Migration Guide

When upgrading between versions:

1. Check `schema_version` field in events
2. Use version-specific parsers if needed
3. Handle deprecated fields gracefully
4. Test with mixed-version data

---

This reference guide is current as of version 1.0.0.
