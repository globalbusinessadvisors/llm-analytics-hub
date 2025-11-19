//! Metrics and Time-Series Examples
//!
//! Comprehensive examples demonstrating metrics aggregation, time-series data models,
//! correlation analysis, and API response formats.

use chrono::{Duration, Utc};
use llm_analytics_hub::models::api::*;
use llm_analytics_hub::models::correlation::*;
use llm_analytics_hub::models::metrics::*;
use llm_analytics_hub::models::timeseries::*;
use llm_analytics_hub::schemas::events::*;
use std::collections::HashMap;
use uuid::Uuid;

fn main() {
    println!("=== Metrics and Time-Series Examples ===\n");

    metrics_examples();
    timeseries_examples();
    correlation_examples();
    api_response_examples();
}

// ============================================================================
// METRICS EXAMPLES
// ============================================================================

fn metrics_examples() {
    println!("--- Metrics Aggregation Examples ---\n");

    // Example 1: Counter Metric
    let mut counter_tags = HashMap::new();
    counter_tags.insert("model".to_string(), "gpt-4".to_string());
    counter_tags.insert("environment".to_string(), "production".to_string());

    let counter = MetricType::Counter(CounterMetric {
        name: "llm_requests_total".to_string(),
        value: 125834,
        rate: Some(45.5),
        tags: counter_tags,
        timestamp: Utc::now(),
    });

    println!("1. Counter Metric:");
    println!("{}\n", serde_json::to_string_pretty(&counter).unwrap());

    // Example 2: Histogram Metric with Statistical Measures
    let buckets = vec![
        HistogramBucket {
            upper_bound: 100.0,
            count: 1500,
        },
        HistogramBucket {
            upper_bound: 500.0,
            count: 3200,
        },
        HistogramBucket {
            upper_bound: 1000.0,
            count: 1800,
        },
        HistogramBucket {
            upper_bound: 5000.0,
            count: 500,
        },
    ];

    let mut histogram_tags = HashMap::new();
    histogram_tags.insert("endpoint".to_string(), "/api/v1/chat".to_string());

    let histogram = MetricType::Histogram(HistogramMetric {
        name: "request_latency_ms".to_string(),
        stats: StatisticalMeasures {
            avg: 450.5,
            min: 50.0,
            max: 4800.0,
            p50: 380.0,
            p95: 1200.0,
            p99: 2500.0,
            stddev: Some(350.2),
            count: 7000,
            sum: 3153500.0,
        },
        buckets,
        tags: histogram_tags,
        timestamp: Utc::now(),
    });

    println!("2. Histogram Metric:");
    println!("{}\n", serde_json::to_string_pretty(&histogram).unwrap());

    // Example 3: Aggregated Metric with Time Window
    let mut agg_tags = HashMap::new();
    agg_tags.insert("model".to_string(), "claude-3".to_string());

    let aggregated = AggregatedMetric {
        name: "token_usage".to_string(),
        window: TimeWindow::OneHour,
        window_start: Utc::now() - Duration::hours(1),
        window_end: Utc::now(),
        values: MetricValues::Stats(StatisticalMeasures {
            avg: 850.5,
            min: 100.0,
            max: 4000.0,
            p50: 750.0,
            p95: 2000.0,
            p99: 3500.0,
            stddev: Some(450.0),
            count: 5000,
            sum: 4252500.0,
        }),
        tags: agg_tags,
    };

    println!("3. Aggregated Metric (1-hour window):");
    println!("{}\n", serde_json::to_string_pretty(&aggregated).unwrap());

    // Example 4: Composite Metric
    let composite = CompositeMetric {
        metric_id: "cost-per-request".to_string(),
        name: "Cost Per Request".to_string(),
        description: "Average cost per API request combining token costs and infrastructure".to_string(),
        source_modules: vec![
            "llm-costops".to_string(),
            "llm-observatory".to_string(),
        ],
        components: vec![
            ComponentMetric {
                name: "total_cost_usd".to_string(),
                source_module: "llm-costops".to_string(),
                value: 125.50,
                weight: Some(1.0),
            },
            ComponentMetric {
                name: "total_requests".to_string(),
                source_module: "llm-observatory".to_string(),
                value: 25000.0,
                weight: Some(1.0),
            },
        ],
        value: 0.00502, // $125.50 / 25000 requests
        formula: "total_cost_usd / total_requests".to_string(),
        window: TimeWindow::OneDay,
        timestamp: Utc::now(),
    };

    println!("4. Composite Metric (Cross-Module):");
    println!("{}\n", serde_json::to_string_pretty(&composite).unwrap());
}

// ============================================================================
// TIME-SERIES EXAMPLES
// ============================================================================

fn timeseries_examples() {
    println!("--- Time-Series Data Model Examples ---\n");

    // Example 1: Time-Series Point with Performance Fields
    let mut tags = TagSet::default();
    tags.source_module = "llm-observatory".to_string();
    tags.environment = "production".to_string();
    tags.model_id = Some("gpt-4".to_string());
    tags.region = Some("us-east-1".to_string());

    let mut custom_perf = HashMap::new();
    custom_perf.insert("cache_hit_rate".to_string(), 0.85);
    custom_perf.insert("queue_depth".to_string(), 12.0);

    let ts_point = TimeSeriesPoint {
        measurement: "llm_performance".to_string(),
        timestamp: Utc::now(),
        tags,
        fields: FieldSet::Performance(PerformanceFields {
            latency_ms: Some(456.8),
            throughput: Some(125.5),
            error_count: Some(3),
            success_count: Some(2497),
            token_count: Some(45000),
            custom: custom_perf,
        }),
        metadata: None,
    };

    println!("1. Time-Series Point (Performance):");
    println!("{}\n", serde_json::to_string_pretty(&ts_point).unwrap());

    // Example 2: Retention Policy
    let retention = RetentionPolicy {
        name: "production_retention".to_string(),
        full_resolution_days: 14,
        downsample_configs: vec![
            DownsampleConfig {
                after_days: 14,
                resolution_minutes: 5,
            },
            DownsampleConfig {
                after_days: 60,
                resolution_minutes: 60,
            },
            DownsampleConfig {
                after_days: 180,
                resolution_minutes: 1440,
            },
        ],
        max_retention_days: 730,
        shard_duration_hours: 24,
    };

    println!("2. Retention Policy:");
    println!("{}\n", serde_json::to_string_pretty(&retention).unwrap());

    // Example 3: Time-Series Query
    let mut tag_filters = HashMap::new();
    tag_filters.insert("source_module".to_string(), "llm-sentinel".to_string());
    tag_filters.insert("environment".to_string(), "production".to_string());

    let query = TimeSeriesQuery {
        measurement: "security_events".to_string(),
        time_range: TimeRange {
            start: Utc::now() - Duration::hours(24),
            end: Utc::now(),
        },
        tag_filters,
        select_fields: vec!["threat_count".to_string(), "severity_score".to_string()],
        aggregation: Some(Aggregation {
            function: AggregationFunction::Mean,
            window: "5m".to_string(),
            fields: vec!["threat_count".to_string()],
        }),
        group_by: vec!["model_id".to_string()],
        fill: Some(FillStrategy::Zero),
        limit: Some(1000),
        offset: None,
    };

    println!("3. Time-Series Query:");
    println!("{}\n", serde_json::to_string_pretty(&query).unwrap());

    // Example 4: Index Configuration
    let index_config = IndexConfig {
        measurement: "llm_metrics".to_string(),
        indexed_tags: vec![
            "source_module".to_string(),
            "environment".to_string(),
            "model_id".to_string(),
            "region".to_string(),
        ],
        shard_keys: vec!["source_module".to_string(), "environment".to_string()],
        time_partitioning: true,
        partition_interval_hours: 24,
    };

    println!("4. Index Configuration:");
    println!("{}\n", serde_json::to_string_pretty(&index_config).unwrap());
}

// ============================================================================
// CORRELATION EXAMPLES
// ============================================================================

fn correlation_examples() {
    println!("--- Event Correlation Examples ---\n");

    // Example 1: Event Correlation with Causal Chain
    let event1 = CorrelatedEvent {
        event_id: Uuid::new_v4(),
        source_module: SourceModule::LlmCostOps,
        event_type: EventType::Cost,
        severity: Severity::Warning,
        timestamp: Utc::now() - Duration::minutes(5),
        role: EventRole::RootCause,
        summary: "Budget threshold exceeded".to_string(),
        metrics: {
            let mut m = HashMap::new();
            m.insert("budget_usage_percent".to_string(), 92.5);
            m
        },
    };

    let event2 = CorrelatedEvent {
        event_id: Uuid::new_v4(),
        source_module: SourceModule::LlmObservatory,
        event_type: EventType::Telemetry,
        severity: Severity::Info,
        timestamp: Utc::now() - Duration::minutes(3),
        role: EventRole::Contributor,
        summary: "Spike in API requests".to_string(),
        metrics: {
            let mut m = HashMap::new();
            m.insert("requests_per_second".to_string(), 250.0);
            m
        },
    };

    let correlation = EventCorrelation {
        correlation_id: CorrelationId::new(),
        correlation_type: CorrelationType::CausalChain,
        events: vec![event1, event2],
        strength: 0.87,
        confidence: 0.92,
        time_window: models::correlation::TimeWindow {
            start: Utc::now() - Duration::minutes(10),
            end: Utc::now(),
        },
        pattern: None,
        detected_at: Utc::now(),
        metadata: HashMap::new(),
    };

    println!("1. Event Correlation (Causal Chain):");
    println!("{}\n", serde_json::to_string_pretty(&correlation).unwrap());

    // Example 2: Anomaly Correlation
    let anomaly1 = AnomalyEvent {
        event_id: Uuid::new_v4(),
        source_module: SourceModule::LlmObservatory,
        anomaly_type: AnomalyType::Spike,
        anomaly_score: 0.91,
        baseline: 150.0,
        observed: 450.0,
        deviation: 200.0,
        timestamp: Utc::now(),
        metric: "request_latency_ms".to_string(),
    };

    let anomaly2 = AnomalyEvent {
        event_id: Uuid::new_v4(),
        source_module: SourceModule::LlmSentinel,
        anomaly_type: AnomalyType::Spike,
        anomaly_score: 0.85,
        baseline: 5.0,
        observed: 45.0,
        deviation: 40.0,
        timestamp: Utc::now(),
        metric: "threat_count".to_string(),
    };

    let anomaly_correlation = AnomalyCorrelation {
        correlation_id: CorrelationId::new(),
        anomalies: vec![anomaly1, anomaly2],
        strength: 0.88,
        root_cause: Some(RootCauseAnalysis {
            root_event_id: Uuid::new_v4(),
            confidence: 0.85,
            causal_chain: vec![
                CausalLink {
                    from_event_id: Uuid::new_v4(),
                    to_event_id: Uuid::new_v4(),
                    relationship: CausalRelationship::DirectCause,
                    strength: 0.9,
                    time_delta_ms: 1500,
                },
            ],
            contributing_factors: vec![
                "Increased user activity".to_string(),
                "Model scaling issues".to_string(),
            ],
            recommendations: vec![
                "Scale up model infrastructure".to_string(),
                "Implement rate limiting".to_string(),
                "Review security policies".to_string(),
            ],
        }),
        impact: ImpactAssessment {
            severity: ImpactSeverity::High,
            affected_modules: vec![
                SourceModule::LlmObservatory,
                SourceModule::LlmSentinel,
            ],
            performance_impact: Some(PerformanceImpact {
                latency_increase_percent: 200.0,
                throughput_decrease_percent: 35.0,
                error_rate_increase_percent: 15.0,
                affected_requests: 12500,
            }),
            cost_impact: Some(CostImpact {
                additional_cost_usd: 450.0,
                cost_increase_percent: 25.0,
                wasted_resources_usd: 150.0,
            }),
            security_impact: Some(SecurityImpact {
                threats_detected: 45,
                vulnerabilities_exposed: 2,
                data_at_risk: false,
                compliance_violations: 0,
            }),
            business_impact: Some(BusinessImpact {
                users_affected: 2500,
                sla_violations: 3,
                revenue_impact_usd: Some(1000.0),
                reputation_risk: ReputationRisk::Medium,
            }),
        },
        detected_at: Utc::now(),
    };

    println!("2. Anomaly Correlation with Impact Assessment:");
    println!("{}\n", serde_json::to_string_pretty(&anomaly_correlation).unwrap());

    // Example 3: Event Graph
    let graph = EventGraph {
        graph_id: "event-graph-123".to_string(),
        time_range: models::correlation::TimeWindow {
            start: Utc::now() - Duration::hours(1),
            end: Utc::now(),
        },
        nodes: vec![
            EventNode {
                node_id: "node-1".to_string(),
                event_id: Uuid::new_v4(),
                source_module: SourceModule::LlmObservatory,
                event_type: EventType::Telemetry,
                timestamp: Utc::now() - Duration::minutes(30),
                attributes: HashMap::new(),
            },
            EventNode {
                node_id: "node-2".to_string(),
                event_id: Uuid::new_v4(),
                source_module: SourceModule::LlmSentinel,
                event_type: EventType::Security,
                timestamp: Utc::now() - Duration::minutes(25),
                attributes: HashMap::new(),
            },
        ],
        edges: vec![
            EventEdge {
                edge_id: "edge-1".to_string(),
                from_node: "node-1".to_string(),
                to_node: "node-2".to_string(),
                relationship_type: EdgeRelationship::Causes,
                weight: 0.85,
                properties: HashMap::new(),
            },
        ],
        metadata: GraphMetadata {
            node_count: 2,
            edge_count: 1,
            connected_components: 1,
            avg_degree: 1.0,
            density: 0.5,
        },
    };

    println!("3. Event Graph:");
    println!("{}\n", serde_json::to_string_pretty(&graph).unwrap());
}

// ============================================================================
// API RESPONSE EXAMPLES
// ============================================================================

fn api_response_examples() {
    println!("--- API Response Examples ---\n");

    // Example 1: Successful API Response
    let success_response = ApiResponse::success(vec![
        "metric-1".to_string(),
        "metric-2".to_string(),
        "metric-3".to_string(),
    ])
    .with_meta(ResponseMetadata {
        request_id: Uuid::new_v4(),
        timestamp: Utc::now(),
        api_version: "1.0.0".to_string(),
        response_time_ms: Some(45),
        extra: HashMap::new(),
    });

    println!("1. Successful API Response:");
    println!("{}\n", serde_json::to_string_pretty(&success_response).unwrap());

    // Example 2: Error Response
    let error = ApiError::not_found("Metric 'invalid-metric' not found")
        .with_details(ErrorDetails {
            trace: None,
            context: Some({
                let mut ctx = HashMap::new();
                ctx.insert("metric_name".to_string(), serde_json::json!("invalid-metric"));
                ctx
            }),
            suggestions: Some(vec![
                "Check the metric name spelling".to_string(),
                "List available metrics with GET /metrics".to_string(),
            ]),
            documentation_url: Some("https://docs.example.com/metrics".to_string()),
        });

    let error_response: ApiResponse<String> = ApiResponse::error(error);

    println!("2. Error Response:");
    println!("{}\n", serde_json::to_string_pretty(&error_response).unwrap());

    // Example 3: Paginated Response
    let data = vec!["item1", "item2", "item3"];
    let pagination = PaginationMetadata::new(2, 3, 15)
        .with_links("https://api.example.com/metrics");

    let paginated_response = PaginatedResponse {
        status: ResponseStatus::Success,
        data: Some(data),
        pagination,
        error: None,
        meta: ResponseMetadata::default(),
    };

    println!("3. Paginated Response:");
    println!("{}\n", serde_json::to_string_pretty(&paginated_response).unwrap());

    // Example 4: Query Result
    let query_result = QueryResult {
        query_id: Uuid::new_v4(),
        status: QueryStatus::Success,
        data: Some(vec![
            AggregatedValue {
                timestamp: Utc::now() - Duration::hours(2),
                avg: 150.5,
                min: 50.0,
                max: 450.0,
                p50: 140.0,
                p95: 380.0,
                p99: 420.0,
                count: 5000,
            },
            AggregatedValue {
                timestamp: Utc::now() - Duration::hours(1),
                avg: 175.2,
                min: 60.0,
                max: 500.0,
                p50: 165.0,
                p95: 420.0,
                p99: 480.0,
                count: 5500,
            },
        ]),
        metrics: QueryMetrics {
            execution_time_ms: 125,
            records_scanned: 10500,
            records_returned: 2,
            bytes_processed: 524288,
            from_cache: false,
            cache_ttl: None,
        },
        warnings: vec![],
    };

    println!("4. Query Result:");
    println!("{}\n", serde_json::to_string_pretty(&query_result).unwrap());

    // Example 5: Batch Response
    let batch_response: BatchResponse<String> = BatchResponse {
        batch_id: Uuid::new_v4(),
        total_items: 3,
        success_count: 2,
        failure_count: 1,
        results: vec![
            BatchItemResult {
                index: 0,
                item_id: Some("item-1".to_string()),
                status: ItemStatus::Success,
                data: Some("processed successfully".to_string()),
                error: None,
            },
            BatchItemResult {
                index: 1,
                item_id: Some("item-2".to_string()),
                status: ItemStatus::Success,
                data: Some("processed successfully".to_string()),
                error: None,
            },
            BatchItemResult {
                index: 2,
                item_id: Some("item-3".to_string()),
                status: ItemStatus::Failed,
                data: None,
                error: Some(ApiError::bad_request("Invalid data format")),
            },
        ],
        status: BatchStatus::PartialSuccess,
    };

    println!("5. Batch Response:");
    println!("{}\n", serde_json::to_string_pretty(&batch_response).unwrap());
}
