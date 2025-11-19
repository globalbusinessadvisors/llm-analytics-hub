//! Analytics Event Schema
//!
//! Unified event schema that accommodates telemetry, security, cost, and governance events
//! from all modules in the LLM ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Schema version for event compatibility and migration
pub const SCHEMA_VERSION: &str = "1.0.0";

/// Common fields present in all analytics events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommonEventFields {
    /// Unique identifier for this event
    #[serde(default = "Uuid::new_v4")]
    pub event_id: Uuid,

    /// ISO 8601 timestamp when the event occurred
    pub timestamp: DateTime<Utc>,

    /// Source module that generated this event
    pub source_module: SourceModule,

    /// Type of event being reported
    pub event_type: EventType,

    /// Correlation ID for tracing related events across modules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,

    /// Parent event ID for hierarchical event relationships
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_event_id: Option<Uuid>,

    /// Schema version for backward compatibility
    #[serde(default = "default_schema_version")]
    pub schema_version: String,

    /// Severity level of the event
    pub severity: Severity,

    /// Environment where the event occurred
    pub environment: String,

    /// Additional custom tags for filtering and grouping
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

fn default_schema_version() -> String {
    SCHEMA_VERSION.to_string()
}

/// Source modules in the LLM ecosystem
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum SourceModule {
    /// LLM-Observatory: Performance and telemetry monitoring
    LlmObservatory,

    /// LLM-Sentinel: Security monitoring and threat detection
    LlmSentinel,

    /// LLM-CostOps: Cost tracking and optimization
    LlmCostOps,

    /// LLM-Governance-Dashboard: Policy and compliance monitoring
    LlmGovernanceDashboard,

    /// LLM-Registry: Asset and model registry
    LlmRegistry,

    /// LLM-Policy-Engine: Policy evaluation and enforcement
    LlmPolicyEngine,

    /// LLM-Analytics-Hub: Self-monitoring events
    LlmAnalyticsHub,
}

/// High-level event type classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// Telemetry and performance events
    Telemetry,

    /// Security-related events
    Security,

    /// Cost and resource consumption events
    Cost,

    /// Governance and compliance events
    Governance,

    /// System lifecycle events
    Lifecycle,

    /// Audit trail events
    Audit,

    /// Alert and notification events
    Alert,
}

/// Event severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

/// Unified analytics event containing common fields and module-specific payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    /// Common fields shared by all events
    #[serde(flatten)]
    pub common: CommonEventFields,

    /// Module-specific event payload
    pub payload: EventPayload,
}

/// Module-specific event payloads
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "payload_type", content = "data")]
pub enum EventPayload {
    /// Telemetry events from LLM-Observatory
    #[serde(rename = "telemetry")]
    Telemetry(TelemetryPayload),

    /// Security events from LLM-Sentinel
    #[serde(rename = "security")]
    Security(SecurityPayload),

    /// Cost events from LLM-CostOps
    #[serde(rename = "cost")]
    Cost(CostPayload),

    /// Governance events from LLM-Governance-Dashboard
    #[serde(rename = "governance")]
    Governance(GovernancePayload),

    /// Generic custom payload
    #[serde(rename = "custom")]
    Custom(CustomPayload),
}

// ============================================================================
// TELEMETRY PAYLOADS (LLM-Observatory)
// ============================================================================

/// Telemetry event payload from LLM-Observatory
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "telemetry_type")]
pub enum TelemetryPayload {
    /// Request latency measurement
    #[serde(rename = "latency")]
    Latency(LatencyMetrics),

    /// Throughput measurement
    #[serde(rename = "throughput")]
    Throughput(ThroughputMetrics),

    /// Error rate tracking
    #[serde(rename = "error_rate")]
    ErrorRate(ErrorRateMetrics),

    /// Token usage statistics
    #[serde(rename = "token_usage")]
    TokenUsage(TokenUsageMetrics),

    /// Model performance metrics
    #[serde(rename = "model_performance")]
    ModelPerformance(ModelPerformanceMetrics),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyMetrics {
    /// Model or service identifier
    pub model_id: String,

    /// Request identifier
    pub request_id: String,

    /// Total latency in milliseconds
    pub total_latency_ms: f64,

    /// Time to first token (TTFT) in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttft_ms: Option<f64>,

    /// Tokens per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_per_second: Option<f64>,

    /// Latency breakdown by component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<LatencyBreakdown>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyBreakdown {
    pub queue_time_ms: f64,
    pub processing_time_ms: f64,
    pub network_time_ms: f64,
    pub other_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputMetrics {
    pub model_id: String,
    pub requests_per_second: f64,
    pub tokens_per_second: f64,
    pub concurrent_requests: u32,
    pub window_duration_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRateMetrics {
    pub model_id: String,
    pub total_requests: u64,
    pub failed_requests: u64,
    pub error_rate_percent: f64,
    pub error_breakdown: HashMap<String, u64>,
    pub window_duration_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsageMetrics {
    pub model_id: String,
    pub request_id: String,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPerformanceMetrics {
    pub model_id: String,
    pub accuracy: Option<f64>,
    pub quality_score: Option<f64>,
    pub user_satisfaction: Option<f64>,
    pub custom_metrics: HashMap<String, f64>,
}

// ============================================================================
// SECURITY PAYLOADS (LLM-Sentinel)
// ============================================================================

/// Security event payload from LLM-Sentinel
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "security_type")]
pub enum SecurityPayload {
    /// Threat detection event
    #[serde(rename = "threat")]
    Threat(ThreatEvent),

    /// Vulnerability detection
    #[serde(rename = "vulnerability")]
    Vulnerability(VulnerabilityEvent),

    /// Compliance violation
    #[serde(rename = "compliance_violation")]
    ComplianceViolation(ComplianceViolationEvent),

    /// Authentication/Authorization event
    #[serde(rename = "auth")]
    Auth(AuthEvent),

    /// Data privacy event
    #[serde(rename = "privacy")]
    Privacy(PrivacyEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEvent {
    pub threat_id: String,
    pub threat_type: ThreatType,
    pub threat_level: ThreatLevel,
    pub source_ip: Option<String>,
    pub target_resource: String,
    pub attack_vector: String,
    pub mitigation_status: MitigationStatus,
    pub indicators_of_compromise: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ThreatType {
    PromptInjection,
    DataExfiltration,
    ModelPoisoning,
    DenialOfService,
    UnauthorizedAccess,
    MaliciousInput,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MitigationStatus {
    Detected,
    Blocked,
    Mitigated,
    Investigating,
    Resolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityEvent {
    pub vulnerability_id: String,
    pub cve_id: Option<String>,
    pub severity_score: f64,
    pub affected_component: String,
    pub description: String,
    pub remediation_status: RemediationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RemediationStatus {
    Identified,
    PatchAvailable,
    Patching,
    Patched,
    Accepted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolationEvent {
    pub violation_id: String,
    pub regulation: String,
    pub requirement: String,
    pub violation_description: String,
    pub affected_data_types: Vec<String>,
    pub remediation_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthEvent {
    pub user_id: String,
    pub action: AuthAction,
    pub resource: String,
    pub success: bool,
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuthAction {
    Login,
    Logout,
    AccessAttempt,
    PermissionDenied,
    TokenGenerated,
    TokenRevoked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyEvent {
    pub data_type: String,
    pub operation: PrivacyOperation,
    pub user_consent: bool,
    pub data_subjects: Vec<String>,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyOperation {
    DataAccess,
    DataCollection,
    DataSharing,
    DataDeletion,
    ConsentUpdate,
}

// ============================================================================
// COST PAYLOADS (LLM-CostOps)
// ============================================================================

/// Cost event payload from LLM-CostOps
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "cost_type")]
pub enum CostPayload {
    /// Token usage cost
    #[serde(rename = "token_cost")]
    TokenCost(TokenCostEvent),

    /// API cost tracking
    #[serde(rename = "api_cost")]
    ApiCost(ApiCostEvent),

    /// Resource consumption
    #[serde(rename = "resource_consumption")]
    ResourceConsumption(ResourceConsumptionEvent),

    /// Budget alert
    #[serde(rename = "budget_alert")]
    BudgetAlert(BudgetAlertEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenCostEvent {
    pub model_id: String,
    pub request_id: String,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub cost_per_prompt_token: f64,
    pub cost_per_completion_token: f64,
    pub total_cost_usd: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCostEvent {
    pub provider: String,
    pub api_endpoint: String,
    pub request_count: u64,
    pub cost_per_request: f64,
    pub total_cost_usd: f64,
    pub billing_period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConsumptionEvent {
    pub resource_type: ResourceType,
    pub resource_id: String,
    pub quantity: f64,
    pub unit: String,
    pub cost_usd: f64,
    pub utilization_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Compute,
    Storage,
    Network,
    Memory,
    Gpu,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetAlertEvent {
    pub budget_id: String,
    pub budget_name: String,
    pub budget_limit_usd: f64,
    pub current_spend_usd: f64,
    pub threshold_percent: f64,
    pub alert_type: BudgetAlertType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BudgetAlertType {
    Warning,
    Critical,
    Exceeded,
}

// ============================================================================
// GOVERNANCE PAYLOADS (LLM-Governance-Dashboard)
// ============================================================================

/// Governance event payload from LLM-Governance-Dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "governance_type")]
pub enum GovernancePayload {
    /// Policy violation event
    #[serde(rename = "policy_violation")]
    PolicyViolation(PolicyViolationEvent),

    /// Audit trail event
    #[serde(rename = "audit_trail")]
    AuditTrail(AuditTrailEvent),

    /// Compliance check result
    #[serde(rename = "compliance_check")]
    ComplianceCheck(ComplianceCheckEvent),

    /// Data lineage tracking
    #[serde(rename = "data_lineage")]
    DataLineage(DataLineageEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyViolationEvent {
    pub policy_id: String,
    pub policy_name: String,
    pub violation_description: String,
    pub violated_rules: Vec<String>,
    pub resource_id: String,
    pub user_id: Option<String>,
    pub severity: PolicyViolationSeverity,
    pub auto_remediated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum PolicyViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTrailEvent {
    pub action: String,
    pub actor: String,
    pub resource_type: String,
    pub resource_id: String,
    pub changes: HashMap<String, serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckEvent {
    pub check_id: String,
    pub framework: String,
    pub controls_checked: Vec<String>,
    pub passed: bool,
    pub findings: Vec<ComplianceFinding>,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFinding {
    pub control_id: String,
    pub status: ComplianceStatus,
    pub description: String,
    pub evidence: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceStatus {
    Pass,
    Fail,
    NotApplicable,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLineageEvent {
    pub data_asset_id: String,
    pub operation: DataOperation,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub transformation: Option<String>,
    pub lineage_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DataOperation {
    Create,
    Read,
    Update,
    Delete,
    Transform,
    Aggregate,
}

// ============================================================================
// CUSTOM PAYLOAD
// ============================================================================

/// Custom payload for extensibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomPayload {
    pub custom_type: String,
    pub data: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_event_serialization() {
        let event = AnalyticsEvent {
            common: CommonEventFields {
                event_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                source_module: SourceModule::LlmObservatory,
                event_type: EventType::Telemetry,
                correlation_id: Some(Uuid::new_v4()),
                parent_event_id: None,
                schema_version: SCHEMA_VERSION.to_string(),
                severity: Severity::Info,
                environment: "production".to_string(),
                tags: HashMap::new(),
            },
            payload: EventPayload::Telemetry(TelemetryPayload::Latency(LatencyMetrics {
                model_id: "gpt-4".to_string(),
                request_id: "req-123".to_string(),
                total_latency_ms: 1523.45,
                ttft_ms: Some(234.12),
                tokens_per_second: Some(45.6),
                breakdown: None,
            })),
        };

        let json = serde_json::to_string_pretty(&event).unwrap();
        assert!(json.contains("telemetry"));
        assert!(json.contains("gpt-4"));
    }

    #[test]
    fn test_security_event_serialization() {
        let event = AnalyticsEvent {
            common: CommonEventFields {
                event_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                source_module: SourceModule::LlmSentinel,
                event_type: EventType::Security,
                correlation_id: None,
                parent_event_id: None,
                schema_version: SCHEMA_VERSION.to_string(),
                severity: Severity::Critical,
                environment: "production".to_string(),
                tags: HashMap::new(),
            },
            payload: EventPayload::Security(SecurityPayload::Threat(ThreatEvent {
                threat_id: "threat-456".to_string(),
                threat_type: ThreatType::PromptInjection,
                threat_level: ThreatLevel::High,
                source_ip: Some("192.168.1.1".to_string()),
                target_resource: "model-endpoint-1".to_string(),
                attack_vector: "malicious prompt".to_string(),
                mitigation_status: MitigationStatus::Blocked,
                indicators_of_compromise: vec!["ioc1".to_string(), "ioc2".to_string()],
            })),
        };

        let json = serde_json::to_string_pretty(&event).unwrap();
        assert!(json.contains("security"));
        assert!(json.contains("prompt_injection"));
    }
}
