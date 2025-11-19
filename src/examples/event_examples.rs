//! Event Schema Examples
//!
//! Comprehensive examples demonstrating the unified analytics event schema
//! for telemetry, security, cost, and governance events.

use chrono::Utc;
use llm_analytics_hub::schemas::events::*;
use std::collections::HashMap;
use uuid::Uuid;

fn main() {
    println!("=== Analytics Event Schema Examples ===\n");

    telemetry_event_examples();
    security_event_examples();
    cost_event_examples();
    governance_event_examples();
}

// ============================================================================
// TELEMETRY EVENTS (LLM-Observatory)
// ============================================================================

fn telemetry_event_examples() {
    println!("--- Telemetry Event Examples ---\n");

    // Example 1: Latency Metrics Event
    let latency_event = AnalyticsEvent {
        common: create_common_fields(
            SourceModule::LlmObservatory,
            EventType::Telemetry,
            Severity::Info,
        ),
        payload: EventPayload::Telemetry(TelemetryPayload::Latency(LatencyMetrics {
            model_id: "gpt-4".to_string(),
            request_id: "req-abc123".to_string(),
            total_latency_ms: 1523.45,
            ttft_ms: Some(234.12),
            tokens_per_second: Some(45.6),
            breakdown: Some(LatencyBreakdown {
                queue_time_ms: 150.0,
                processing_time_ms: 1200.5,
                network_time_ms: 172.95,
                other_ms: 0.0,
            }),
        })),
    };

    println!("1. Latency Metrics Event:");
    println!("{}\n", serde_json::to_string_pretty(&latency_event).unwrap());

    // Example 2: Throughput Metrics Event
    let throughput_event = AnalyticsEvent {
        common: create_common_fields(
            SourceModule::LlmObservatory,
            EventType::Telemetry,
            Severity::Info,
        ),
        payload: EventPayload::Telemetry(TelemetryPayload::Throughput(ThroughputMetrics {
            model_id: "claude-3".to_string(),
            requests_per_second: 125.5,
            tokens_per_second: 15000.0,
            concurrent_requests: 50,
            window_duration_seconds: 60,
        })),
    };

    println!("2. Throughput Metrics Event:");
    println!("{}\n", serde_json::to_string_pretty(&throughput_event).unwrap());

    // Example 3: Error Rate Metrics Event
    let mut error_breakdown = HashMap::new();
    error_breakdown.insert("timeout".to_string(), 5);
    error_breakdown.insert("rate_limit".to_string(), 3);
    error_breakdown.insert("invalid_request".to_string(), 2);

    let error_rate_event = AnalyticsEvent {
        common: create_common_fields(
            SourceModule::LlmObservatory,
            EventType::Telemetry,
            Severity::Warning,
        ),
        payload: EventPayload::Telemetry(TelemetryPayload::ErrorRate(ErrorRateMetrics {
            model_id: "gpt-3.5-turbo".to_string(),
            total_requests: 1000,
            failed_requests: 10,
            error_rate_percent: 1.0,
            error_breakdown,
            window_duration_seconds: 300,
        })),
    };

    println!("3. Error Rate Metrics Event:");
    println!("{}\n", serde_json::to_string_pretty(&error_rate_event).unwrap());
}

// ============================================================================
// SECURITY EVENTS (LLM-Sentinel)
// ============================================================================

fn security_event_examples() {
    println!("--- Security Event Examples ---\n");

    // Example 1: Threat Detection Event
    let threat_event = AnalyticsEvent {
        common: create_common_fields(
            SourceModule::LlmSentinel,
            EventType::Security,
            Severity::Critical,
        ),
        payload: EventPayload::Security(SecurityPayload::Threat(ThreatEvent {
            threat_id: "threat-789xyz".to_string(),
            threat_type: ThreatType::PromptInjection,
            threat_level: ThreatLevel::High,
            source_ip: Some("192.168.1.100".to_string()),
            target_resource: "api/v1/chat/completions".to_string(),
            attack_vector: "Malicious prompt attempting to override system instructions".to_string(),
            mitigation_status: MitigationStatus::Blocked,
            indicators_of_compromise: vec![
                "ignore previous instructions".to_string(),
                "system override".to_string(),
            ],
        })),
    };

    println!("1. Threat Detection Event:");
    println!("{}\n", serde_json::to_string_pretty(&threat_event).unwrap());

    // Example 2: Vulnerability Event
    let vulnerability_event = AnalyticsEvent {
        common: create_common_fields(
            SourceModule::LlmSentinel,
            EventType::Security,
            Severity::Error,
        ),
        payload: EventPayload::Security(SecurityPayload::Vulnerability(VulnerabilityEvent {
            vulnerability_id: "vuln-456".to_string(),
            cve_id: Some("CVE-2024-12345".to_string()),
            severity_score: 7.5,
            affected_component: "authentication-module".to_string(),
            description: "Potential authentication bypass vulnerability detected".to_string(),
            remediation_status: RemediationStatus::PatchAvailable,
        })),
    };

    println!("2. Vulnerability Event:");
    println!("{}\n", serde_json::to_string_pretty(&vulnerability_event).unwrap());

    // Example 3: Authentication Event
    let auth_event = AnalyticsEvent {
        common: create_common_fields(
            SourceModule::LlmSentinel,
            EventType::Security,
            Severity::Warning,
        ),
        payload: EventPayload::Security(SecurityPayload::Auth(AuthEvent {
            user_id: "user-123".to_string(),
            action: AuthAction::PermissionDenied,
            resource: "sensitive-model-endpoint".to_string(),
            success: false,
            failure_reason: Some("Insufficient privileges".to_string()),
        })),
    };

    println!("3. Authentication Event:");
    println!("{}\n", serde_json::to_string_pretty(&auth_event).unwrap());
}

// ============================================================================
// COST EVENTS (LLM-CostOps)
// ============================================================================

fn cost_event_examples() {
    println!("--- Cost Event Examples ---\n");

    // Example 1: Token Cost Event
    let token_cost_event = AnalyticsEvent {
        common: create_common_fields(
            SourceModule::LlmCostOps,
            EventType::Cost,
            Severity::Info,
        ),
        payload: EventPayload::Cost(CostPayload::TokenCost(TokenCostEvent {
            model_id: "gpt-4".to_string(),
            request_id: "req-cost-123".to_string(),
            prompt_tokens: 500,
            completion_tokens: 300,
            total_tokens: 800,
            cost_per_prompt_token: 0.00003,
            cost_per_completion_token: 0.00006,
            total_cost_usd: 0.033,
            currency: "USD".to_string(),
        })),
    };

    println!("1. Token Cost Event:");
    println!("{}\n", serde_json::to_string_pretty(&token_cost_event).unwrap());

    // Example 2: Budget Alert Event
    let budget_alert_event = AnalyticsEvent {
        common: create_common_fields(
            SourceModule::LlmCostOps,
            EventType::Alert,
            Severity::Warning,
        ),
        payload: EventPayload::Cost(CostPayload::BudgetAlert(BudgetAlertEvent {
            budget_id: "budget-monthly-2024".to_string(),
            budget_name: "Monthly LLM API Budget".to_string(),
            budget_limit_usd: 10000.0,
            current_spend_usd: 8500.0,
            threshold_percent: 85.0,
            alert_type: BudgetAlertType::Warning,
        })),
    };

    println!("2. Budget Alert Event:");
    println!("{}\n", serde_json::to_string_pretty(&budget_alert_event).unwrap());

    // Example 3: Resource Consumption Event
    let resource_event = AnalyticsEvent {
        common: create_common_fields(
            SourceModule::LlmCostOps,
            EventType::Cost,
            Severity::Info,
        ),
        payload: EventPayload::Cost(CostPayload::ResourceConsumption(ResourceConsumptionEvent {
            resource_type: ResourceType::Gpu,
            resource_id: "gpu-cluster-us-east-1".to_string(),
            quantity: 24.5,
            unit: "GPU-hours".to_string(),
            cost_usd: 245.0,
            utilization_percent: 87.5,
        })),
    };

    println!("3. Resource Consumption Event:");
    println!("{}\n", serde_json::to_string_pretty(&resource_event).unwrap());
}

// ============================================================================
// GOVERNANCE EVENTS (LLM-Governance-Dashboard)
// ============================================================================

fn governance_event_examples() {
    println!("--- Governance Event Examples ---\n");

    // Example 1: Policy Violation Event
    let policy_violation_event = AnalyticsEvent {
        common: create_common_fields(
            SourceModule::LlmGovernanceDashboard,
            EventType::Governance,
            Severity::Error,
        ),
        payload: EventPayload::Governance(GovernancePayload::PolicyViolation(PolicyViolationEvent {
            policy_id: "pol-data-privacy-001".to_string(),
            policy_name: "PII Detection Policy".to_string(),
            violation_description: "Potential PII detected in model output".to_string(),
            violated_rules: vec![
                "no-email-addresses".to_string(),
                "no-phone-numbers".to_string(),
            ],
            resource_id: "model-response-456".to_string(),
            user_id: Some("user-789".to_string()),
            severity: PolicyViolationSeverity::High,
            auto_remediated: true,
        })),
    };

    println!("1. Policy Violation Event:");
    println!("{}\n", serde_json::to_string_pretty(&policy_violation_event).unwrap());

    // Example 2: Audit Trail Event
    let mut changes = HashMap::new();
    changes.insert("model_id".to_string(), serde_json::json!({
        "old": "gpt-3.5-turbo",
        "new": "gpt-4"
    }));
    changes.insert("temperature".to_string(), serde_json::json!({
        "old": 0.7,
        "new": 0.5
    }));

    let audit_event = AnalyticsEvent {
        common: create_common_fields(
            SourceModule::LlmGovernanceDashboard,
            EventType::Audit,
            Severity::Info,
        ),
        payload: EventPayload::Governance(GovernancePayload::AuditTrail(AuditTrailEvent {
            action: "update_configuration".to_string(),
            actor: "admin@example.com".to_string(),
            resource_type: "llm_endpoint".to_string(),
            resource_id: "endpoint-chat-001".to_string(),
            changes,
            ip_address: Some("10.0.1.50".to_string()),
            user_agent: Some("Mozilla/5.0 (compatible)".to_string()),
        })),
    };

    println!("2. Audit Trail Event:");
    println!("{}\n", serde_json::to_string_pretty(&audit_event).unwrap());

    // Example 3: Compliance Check Event
    let compliance_event = AnalyticsEvent {
        common: create_common_fields(
            SourceModule::LlmGovernanceDashboard,
            EventType::Governance,
            Severity::Info,
        ),
        payload: EventPayload::Governance(GovernancePayload::ComplianceCheck(ComplianceCheckEvent {
            check_id: "compliance-gdpr-2024-01".to_string(),
            framework: "GDPR".to_string(),
            controls_checked: vec![
                "data-minimization".to_string(),
                "purpose-limitation".to_string(),
                "storage-limitation".to_string(),
            ],
            passed: false,
            findings: vec![
                ComplianceFinding {
                    control_id: "storage-limitation".to_string(),
                    status: ComplianceStatus::Fail,
                    description: "Data retained beyond specified period".to_string(),
                    evidence: Some("Logs older than 90 days found".to_string()),
                },
            ],
            score: 66.7,
        })),
    };

    println!("3. Compliance Check Event:");
    println!("{}\n", serde_json::to_string_pretty(&compliance_event).unwrap());
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn create_common_fields(
    source: SourceModule,
    event_type: EventType,
    severity: Severity,
) -> CommonEventFields {
    CommonEventFields {
        event_id: Uuid::new_v4(),
        timestamp: Utc::now(),
        source_module: source,
        event_type,
        correlation_id: None,
        parent_event_id: None,
        schema_version: "1.0.0".to_string(),
        severity,
        environment: "production".to_string(),
        tags: HashMap::new(),
    }
}
