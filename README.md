# LLM Analytics Hub

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

Centralized analytics hub for the LLM ecosystem, providing comprehensive data models and schemas for telemetry, security, cost, and governance monitoring across multiple LLM modules.

## Overview

The LLM Analytics Hub serves as the central data modeling layer for a distributed LLM monitoring ecosystem. It provides unified schemas and data models that enable:

- **Unified Event Ingestion**: Single schema for events from LLM-Observatory, LLM-Sentinel, LLM-CostOps, and LLM-Governance-Dashboard
- **Time-Series Analytics**: Optimized data models for high-performance time-series queries
- **Cross-Module Correlation**: Event correlation and anomaly detection across modules
- **Flexible Aggregation**: Multiple time windows with comprehensive statistical measures
- **Rich Metadata**: Asset, policy, dashboard, and user preference schemas

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Source Modules                              │
├──────────────┬──────────────┬──────────────┬──────────────────┤
│ Observatory  │  Sentinel    │  CostOps     │  Governance      │
│ (Telemetry)  │  (Security)  │  (Cost)      │  (Policy)        │
└──────┬───────┴──────┬───────┴──────┬───────┴──────┬───────────┘
       │              │              │              │
       └──────────────┴──────────────┴──────────────┘
                         │
                         ▼
       ┌─────────────────────────────────────────┐
       │       LLM Analytics Hub                 │
       │  ┌─────────────────────────────────┐   │
       │  │   Unified Event Schema          │   │
       │  └─────────────────────────────────┘   │
       │  ┌─────────────────────────────────┐   │
       │  │   Metrics Aggregation           │   │
       │  └─────────────────────────────────┘   │
       │  ┌─────────────────────────────────┐   │
       │  │   Time-Series Models            │   │
       │  └─────────────────────────────────┘   │
       │  ┌─────────────────────────────────┐   │
       │  │   Correlation Engine            │   │
       │  └─────────────────────────────────┘   │
       └─────────────────────────────────────────┘
                         │
                         ▼
       ┌─────────────────────────────────────────┐
       │      Storage & Query Layer              │
       │  (InfluxDB, PostgreSQL, etc.)          │
       └─────────────────────────────────────────┘
```

## Features

### 1. Analytics Event Schema

Unified event schema accommodating all module types:

- **Telemetry Events** (LLM-Observatory): Latency, throughput, error rates, token usage
- **Security Events** (LLM-Sentinel): Threats, vulnerabilities, compliance violations
- **Cost Events** (LLM-CostOps): Token costs, API costs, resource consumption
- **Governance Events** (LLM-Governance-Dashboard): Policy violations, audit trails

Common fields include:
- Event ID and timestamp
- Source module and event type
- Correlation ID for distributed tracing
- Severity levels
- Custom tags

### 2. Metrics Aggregation Models

- **Time Windows**: 1m, 5m, 15m, 1h, 6h, 1d, 1w, 1M
- **Statistical Measures**: avg, min, max, p50, p95, p99, stddev, count, sum
- **Metric Types**: Counter, Gauge, Histogram, Summary
- **Composite Metrics**: Cross-module metrics (cost per request, error-adjusted throughput)

### 3. Time-Series Data Models

- **Tag-Based Organization**: Low-cardinality tags for efficient indexing
- **Field Selection**: High-cardinality measurements
- **Retention Policies**: Multi-tier retention with automatic downsampling
- **Query Optimization**: Indexed tags, time partitioning, shard keys

### 4. Correlation Schemas

- **Event Correlation**: Causal chains, temporal correlations, pattern matching
- **Anomaly Detection**: Spike, drop, pattern deviation, frequency anomalies
- **Root Cause Analysis**: Automated causal chain identification
- **Impact Assessment**: Performance, cost, security, business impact

### 5. Metadata Schemas

- **Asset Metadata**: Models, prompts, datasets, endpoints, applications
- **Policy Definitions**: Security, compliance, cost control, performance policies
- **Dashboard Configuration**: Widgets, layouts, visualization settings
- **User Preferences**: Display settings, notifications, saved queries

### 6. API Response Models

- **Standard Responses**: Consistent success/error format
- **Pagination**: Cursor and offset-based with navigation links
- **Error Handling**: Detailed error responses with suggestions
- **Streaming**: Server-Sent Events (SSE) support
- **Batch Operations**: Bulk operation responses

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
llm-analytics-hub = "0.1.0"
```

Or clone the repository:

```bash
git clone https://github.com/your-org/llm-analytics-hub.git
cd llm-analytics-hub
```

### Building

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Running Examples

```bash
# Event schema examples
cargo run --example event_examples

# Metrics and time-series examples
cargo run --example metrics_examples
```

## Usage Examples

### Creating a Telemetry Event

```rust
use llm_analytics_hub::*;
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;

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
println!("{}", json);
```

### Recording Metrics

```rust
use llm_analytics_hub::models::metrics::*;
use chrono::Utc;
use std::collections::HashMap;

let mut tags = HashMap::new();
tags.insert("model".to_string(), "gpt-4".to_string());
tags.insert("environment".to_string(), "production".to_string());

let counter = MetricType::Counter(CounterMetric {
    name: "llm_requests_total".to_string(),
    value: 125834,
    rate: Some(45.5),
    tags,
    timestamp: Utc::now(),
});
```

### Querying Time-Series Data

```rust
use llm_analytics_hub::models::timeseries::*;
use chrono::{Duration, Utc};
use std::collections::HashMap;

let mut tag_filters = HashMap::new();
tag_filters.insert("model_id".to_string(), "gpt-4".to_string());

let query = TimeSeriesQuery {
    measurement: "llm_latency".to_string(),
    time_range: TimeRange {
        start: Utc::now() - Duration::hours(24),
        end: Utc::now(),
    },
    tag_filters,
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

## Documentation

- **[Data Models Documentation](DATA_MODELS_DOCUMENTATION.md)**: Comprehensive guide to all schemas and models
- **[Schema Reference](SCHEMA_REFERENCE.md)**: Quick reference for schemas, field types, and examples
- **[API Documentation](https://docs.rs/llm-analytics-hub)**: Generated API documentation

## Project Structure

```
llm-analytics-hub/
├── src/
│   ├── lib.rs                    # Library entry point
│   ├── schemas/
│   │   ├── events.rs             # Analytics event schema
│   │   └── metadata.rs           # Metadata schemas
│   ├── models/
│   │   ├── metrics.rs            # Metrics aggregation models
│   │   ├── timeseries.rs         # Time-series data models
│   │   ├── correlation.rs        # Correlation schemas
│   │   └── api.rs                # API response models
│   └── examples/
│       ├── event_examples.rs     # Event schema examples
│       └── metrics_examples.rs   # Metrics examples
├── Cargo.toml                    # Project configuration
├── README.md                     # This file
├── DATA_MODELS_DOCUMENTATION.md  # Detailed documentation
├── SCHEMA_REFERENCE.md           # Quick reference guide
└── LICENSE                       # Apache 2.0 License
```

## Schema Versioning

The project follows semantic versioning for schemas:

- **Current Version**: 1.0.0
- **Schema Version Field**: All events include `schema_version` for compatibility
- **Migration Strategy**: Backward compatibility maintained for minor versions

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Write tests for new features
- Update documentation for API changes
- Follow Rust naming conventions
- Run `cargo fmt` and `cargo clippy` before committing
- Maintain backward compatibility when possible

## Testing

Run the complete test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test models::metrics

# Run benchmarks (if available)
cargo bench
```

## Performance Considerations

### Time-Series Optimization

- Keep tag cardinality low (< 1000 unique values per tag)
- Index only frequently queried tags
- Use appropriate time windows for aggregations
- Implement retention policies to manage storage

### Event Processing

- Batch events for efficient ingestion
- Use correlation IDs for distributed tracing
- Set appropriate severity levels for filtering
- Include relevant tags for dimensional analysis

## Roadmap

- [ ] GraphQL API support
- [ ] Protocol Buffers serialization
- [ ] OpenTelemetry integration
- [ ] Real-time streaming aggregations
- [ ] ML-based anomaly detection
- [ ] Advanced correlation algorithms
- [ ] Multi-cloud deployment templates

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

This project is part of the LLM ecosystem monitoring suite, working alongside:

- **LLM-Observatory**: Performance and telemetry monitoring
- **LLM-Sentinel**: Security threat detection and monitoring
- **LLM-CostOps**: Cost tracking and optimization
- **LLM-Governance-Dashboard**: Policy and compliance monitoring
- **LLM-Registry**: Asset and model registry
- **LLM-Policy-Engine**: Policy evaluation and enforcement

## Support

For questions, issues, or feature requests:

- Open an issue on GitHub
- Check the documentation
- Review example code

## Status

**Current Version**: 0.1.0 (Development)

This is an initial release focused on core data modeling capabilities. Production-ready features are being actively developed.
