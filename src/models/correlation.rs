//! Correlation Schemas
//!
//! Cross-module event correlation patterns, anomaly correlation, causality tracking,
//! and graph-based relationship models for understanding system behavior.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::schemas::events::{EventType, Severity, SourceModule};

/// Correlation identifier linking related events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CorrelationId(pub Uuid);

impl CorrelationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}

/// Event correlation representing relationships between events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCorrelation {
    /// Unique correlation identifier
    pub correlation_id: CorrelationId,

    /// Type of correlation
    pub correlation_type: CorrelationType,

    /// Events involved in this correlation
    pub events: Vec<CorrelatedEvent>,

    /// Correlation strength (0.0 to 1.0)
    pub strength: f64,

    /// Confidence level of the correlation (0.0 to 1.0)
    pub confidence: f64,

    /// Time window of the correlation
    pub time_window: TimeWindow,

    /// Correlation pattern matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<CorrelationPattern>,

    /// Timestamp when correlation was identified
    pub detected_at: DateTime<Utc>,

    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl TimeWindow {
    pub fn duration_seconds(&self) -> i64 {
        (self.end - self.start).num_seconds()
    }
}

/// Types of correlations between events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CorrelationType {
    /// Events caused by the same root cause
    CausalChain,

    /// Events occurring simultaneously
    Temporal,

    /// Similar patterns across different modules
    PatternMatch,

    /// Anomalous behavior correlation
    Anomaly,

    /// Cost impact correlation
    CostImpact,

    /// Security incident correlation
    SecurityIncident,

    /// Performance degradation chain
    PerformanceDegradation,

    /// Compliance violation cascade
    ComplianceCascade,
}

/// Individual event in a correlation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelatedEvent {
    /// Event identifier
    pub event_id: Uuid,

    /// Source module
    pub source_module: SourceModule,

    /// Event type
    pub event_type: EventType,

    /// Event severity
    pub severity: Severity,

    /// Event timestamp
    pub timestamp: DateTime<Utc>,

    /// Role in the correlation
    pub role: EventRole,

    /// Event summary/description
    pub summary: String,

    /// Key metrics from the event
    #[serde(default)]
    pub metrics: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventRole {
    /// Root cause or trigger event
    RootCause,

    /// Contributing factor
    Contributor,

    /// Symptom or effect
    Effect,

    /// Related but not causal
    Related,
}

/// Predefined correlation patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationPattern {
    /// Pattern identifier
    pub pattern_id: String,

    /// Pattern name
    pub name: String,

    /// Pattern description
    pub description: String,

    /// Expected modules involved
    pub modules: Vec<SourceModule>,

    /// Expected event sequence
    pub sequence: Vec<PatternStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternStep {
    pub step_number: u32,
    pub module: SourceModule,
    pub event_type: EventType,
    pub time_offset_ms: Option<i64>, // Relative to previous step
    pub conditions: HashMap<String, String>,
}

/// Anomaly correlation for detecting related anomalies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyCorrelation {
    /// Correlation identifier
    pub correlation_id: CorrelationId,

    /// Detected anomalies
    pub anomalies: Vec<AnomalyEvent>,

    /// Correlation strength
    pub strength: f64,

    /// Root cause analysis result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause: Option<RootCauseAnalysis>,

    /// Impact assessment
    pub impact: ImpactAssessment,

    /// Detection timestamp
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyEvent {
    /// Event identifier
    pub event_id: Uuid,

    /// Source module
    pub source_module: SourceModule,

    /// Anomaly type
    pub anomaly_type: AnomalyType,

    /// Anomaly score (0.0 to 1.0, higher = more anomalous)
    pub anomaly_score: f64,

    /// Baseline value
    pub baseline: f64,

    /// Observed value
    pub observed: f64,

    /// Deviation from baseline
    pub deviation: f64,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Affected metric
    pub metric: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AnomalyType {
    /// Value exceeds expected range
    Spike,

    /// Value below expected range
    Drop,

    /// Unusual pattern or trend
    PatternDeviation,

    /// Unexpected frequency
    FrequencyAnomaly,

    /// Distribution shift
    DistributionShift,
}

/// Root cause analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootCauseAnalysis {
    /// Identified root cause event
    pub root_event_id: Uuid,

    /// Confidence in root cause identification (0.0 to 1.0)
    pub confidence: f64,

    /// Causal chain from root to effects
    pub causal_chain: Vec<CausalLink>,

    /// Contributing factors
    pub contributing_factors: Vec<String>,

    /// Recommended actions
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    pub from_event_id: Uuid,
    pub to_event_id: Uuid,
    pub relationship: CausalRelationship,
    pub strength: f64,
    pub time_delta_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CausalRelationship {
    DirectCause,
    IndirectCause,
    Correlation,
    Amplification,
}

/// Impact assessment of correlated events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    /// Overall impact severity
    pub severity: ImpactSeverity,

    /// Affected modules
    pub affected_modules: Vec<SourceModule>,

    /// Performance impact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_impact: Option<PerformanceImpact>,

    /// Cost impact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_impact: Option<CostImpact>,

    /// Security impact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_impact: Option<SecurityImpact>,

    /// Business impact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_impact: Option<BusinessImpact>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum ImpactSeverity {
    Negligible,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceImpact {
    pub latency_increase_percent: f64,
    pub throughput_decrease_percent: f64,
    pub error_rate_increase_percent: f64,
    pub affected_requests: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostImpact {
    pub additional_cost_usd: f64,
    pub cost_increase_percent: f64,
    pub wasted_resources_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityImpact {
    pub threats_detected: u64,
    pub vulnerabilities_exposed: u64,
    pub data_at_risk: bool,
    pub compliance_violations: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessImpact {
    pub users_affected: u64,
    pub sla_violations: u64,
    pub revenue_impact_usd: Option<f64>,
    pub reputation_risk: ReputationRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReputationRisk {
    None,
    Low,
    Medium,
    High,
    Severe,
}

/// Graph-based event relationship model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGraph {
    /// Graph identifier
    pub graph_id: String,

    /// Graph time range
    pub time_range: TimeWindow,

    /// Nodes (events)
    pub nodes: Vec<EventNode>,

    /// Edges (relationships)
    pub edges: Vec<EventEdge>,

    /// Graph metadata
    pub metadata: GraphMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventNode {
    pub node_id: String,
    pub event_id: Uuid,
    pub source_module: SourceModule,
    pub event_type: EventType,
    pub timestamp: DateTime<Utc>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEdge {
    pub edge_id: String,
    pub from_node: String,
    pub to_node: String,
    pub relationship_type: EdgeRelationship,
    pub weight: f64,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMetadata {
    pub node_count: usize,
    pub edge_count: usize,
    pub connected_components: usize,
    pub avg_degree: f64,
    pub density: f64,
}

/// Correlation query for finding related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationQuery {
    /// Seed event to find correlations for
    pub seed_event_id: Uuid,

    /// Time window to search within
    pub time_window_minutes: i64,

    /// Minimum correlation strength
    #[serde(default = "default_min_strength")]
    pub min_strength: f64,

    /// Correlation types to include
    #[serde(default)]
    pub correlation_types: Vec<CorrelationType>,

    /// Modules to include in search
    #[serde(default)]
    pub include_modules: Vec<SourceModule>,

    /// Maximum depth for causal chain
    #[serde(default = "default_max_depth")]
    pub max_depth: u32,
}

fn default_min_strength() -> f64 {
    0.7
}

fn default_max_depth() -> u32 {
    5
}

/// Cross-module correlation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationConfig {
    /// Enable automatic correlation detection
    pub auto_detect: bool,

    /// Correlation patterns to monitor
    pub patterns: Vec<CorrelationPattern>,

    /// Time window for correlation (minutes)
    pub correlation_window_minutes: i64,

    /// Minimum events required for correlation
    pub min_events: usize,

    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub min_correlation_strength: f64,
    pub min_anomaly_score: f64,
    pub critical_impact_threshold: f64,
}

impl Default for CorrelationConfig {
    fn default() -> Self {
        Self {
            auto_detect: true,
            patterns: Vec::new(),
            correlation_window_minutes: 60,
            min_events: 2,
            alert_thresholds: AlertThresholds {
                min_correlation_strength: 0.8,
                min_anomaly_score: 0.7,
                critical_impact_threshold: 0.9,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlation_id_creation() {
        let id1 = CorrelationId::new();
        let id2 = CorrelationId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_time_window_duration() {
        let start = Utc::now();
        let end = start + chrono::Duration::minutes(30);
        let window = TimeWindow { start, end };
        assert_eq!(window.duration_seconds(), 1800);
    }

    #[test]
    fn test_event_correlation_serialization() {
        let correlation = EventCorrelation {
            correlation_id: CorrelationId::new(),
            correlation_type: CorrelationType::CausalChain,
            events: vec![],
            strength: 0.85,
            confidence: 0.9,
            time_window: TimeWindow {
                start: Utc::now(),
                end: Utc::now() + chrono::Duration::minutes(10),
            },
            pattern: None,
            detected_at: Utc::now(),
            metadata: HashMap::new(),
        };

        let json = serde_json::to_string_pretty(&correlation).unwrap();
        assert!(json.contains("causal_chain"));
        assert!(json.contains("0.85"));
    }
}
