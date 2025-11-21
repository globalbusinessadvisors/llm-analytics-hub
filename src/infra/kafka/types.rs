//! Kafka type definitions and configurations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Topic configuration for LLM Analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicConfig {
    /// Topic name
    pub name: String,

    /// Number of partitions
    pub partitions: i32,

    /// Replication factor
    pub replication_factor: i32,

    /// Topic-level configurations
    pub config: HashMap<String, String>,

    /// Human-readable description
    pub description: String,
}

impl TopicConfig {
    /// Create a new topic configuration
    pub fn new(
        name: impl Into<String>,
        partitions: i32,
        replication_factor: i32,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            partitions,
            replication_factor,
            config: HashMap::new(),
            description: description.into(),
        }
    }

    /// Add a configuration parameter
    pub fn with_config(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.insert(key.into(), value.into());
        self
    }

    /// Add retention in milliseconds
    pub fn with_retention_ms(self, ms: u64) -> Self {
        self.with_config("retention.ms", ms.to_string())
    }

    /// Add retention in bytes
    pub fn with_retention_bytes(self, bytes: u64) -> Self {
        self.with_config("retention.bytes", bytes.to_string())
    }

    /// Set cleanup policy
    pub fn with_cleanup_policy(self, policy: &str) -> Self {
        self.with_config("cleanup.policy", policy)
    }

    /// Set compression type
    pub fn with_compression(self, compression: &str) -> Self {
        self.with_config("compression.type", compression)
    }

    /// Set minimum in-sync replicas
    pub fn with_min_isr(self, min_isr: i32) -> Self {
        self.with_config("min.insync.replicas", min_isr.to_string())
    }
}

/// Get all LLM Analytics topic configurations
pub fn get_llm_topic_configs() -> Vec<TopicConfig> {
    vec![
        TopicConfig::new("llm-events", 32, 3, "Main event stream for all LLM events")
            .with_cleanup_policy("delete")
            .with_retention_ms(604_800_000) // 7 days
            .with_retention_bytes(536_870_912_000) // 500GB
            .with_config("segment.ms", "86400000") // 1 day
            .with_config("segment.bytes", "1073741824") // 1GB
            .with_compression("lz4")
            .with_min_isr(2)
            .with_config("max.message.bytes", "10485760"), // 10MB

        TopicConfig::new("llm-metrics", 32, 3, "Performance metrics and telemetry data")
            .with_cleanup_policy("delete")
            .with_retention_ms(2_592_000_000) // 30 days
            .with_retention_bytes(1_073_741_824_000) // 1TB
            .with_config("segment.ms", "86400000")
            .with_config("segment.bytes", "1073741824")
            .with_compression("lz4")
            .with_min_isr(2),

        TopicConfig::new("llm-analytics", 16, 3, "Processed analytics and aggregated data")
            .with_cleanup_policy("delete")
            .with_retention_ms(604_800_000) // 7 days
            .with_compression("lz4")
            .with_min_isr(2),

        TopicConfig::new("llm-traces", 32, 3, "Distributed tracing data")
            .with_cleanup_policy("delete")
            .with_retention_ms(604_800_000) // 7 days
            .with_compression("lz4")
            .with_min_isr(2),

        TopicConfig::new("llm-errors", 16, 3, "Error events and exceptions")
            .with_cleanup_policy("delete")
            .with_retention_ms(2_592_000_000) // 30 days
            .with_compression("lz4")
            .with_min_isr(2),

        TopicConfig::new("llm-audit", 8, 3, "Audit logs with compaction for compliance")
            .with_cleanup_policy("compact,delete")
            .with_retention_ms(7_776_000_000) // 90 days
            .with_compression("lz4")
            .with_min_isr(2)
            .with_config("min.compaction.lag.ms", "86400000"), // 1 day

        TopicConfig::new("llm-aggregated-metrics", 16, 3, "Pre-aggregated metrics for faster queries")
            .with_cleanup_policy("delete")
            .with_retention_ms(2_592_000_000) // 30 days
            .with_compression("lz4")
            .with_min_isr(2),

        TopicConfig::new("llm-alerts", 8, 3, "Alert notifications and triggers")
            .with_cleanup_policy("delete")
            .with_retention_ms(604_800_000) // 7 days
            .with_compression("lz4")
            .with_min_isr(2),

        TopicConfig::new("llm-usage-stats", 16, 3, "Usage statistics with compaction")
            .with_cleanup_policy("compact,delete")
            .with_retention_ms(2_592_000_000) // 30 days
            .with_compression("lz4")
            .with_min_isr(2),

        TopicConfig::new("llm-model-performance", 16, 3, "Model performance metrics and benchmarks")
            .with_cleanup_policy("delete")
            .with_retention_ms(2_592_000_000) // 30 days
            .with_compression("lz4")
            .with_min_isr(2),

        TopicConfig::new("llm-cost-tracking", 8, 3, "Cost analysis and tracking data")
            .with_cleanup_policy("delete")
            .with_retention_ms(7_776_000_000) // 90 days
            .with_compression("lz4")
            .with_min_isr(2),

        TopicConfig::new("llm-session-events", 16, 3, "Session-level events and state changes")
            .with_cleanup_policy("delete")
            .with_retention_ms(604_800_000) // 7 days
            .with_compression("lz4")
            .with_min_isr(2),

        TopicConfig::new("llm-user-feedback", 8, 3, "User feedback and ratings")
            .with_cleanup_policy("delete")
            .with_retention_ms(7_776_000_000) // 90 days
            .with_compression("lz4")
            .with_min_isr(2),

        TopicConfig::new("llm-system-health", 8, 3, "System health metrics and status")
            .with_cleanup_policy("delete")
            .with_retention_ms(2_592_000_000) // 30 days
            .with_compression("lz4")
            .with_min_isr(2),
    ]
}

/// ACL resource type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AclResourceType {
    /// Topic resource
    Topic,
    /// Consumer group resource
    Group,
    /// Cluster resource
    Cluster,
}

/// ACL operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AclOperation {
    /// Read operation
    Read,
    /// Write operation
    Write,
    /// Describe operation
    Describe,
    /// Create operation
    Create,
    /// Delete operation
    Delete,
    /// All operations
    All,
}

/// ACL permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AclPermission {
    /// Allow permission
    Allow,
    /// Deny permission
    Deny,
}

/// ACL configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclConfig {
    /// Principal (User:username)
    pub principal: String,

    /// Resource type
    pub resource_type: AclResourceType,

    /// Resource name (supports wildcards)
    pub resource_name: String,

    /// Operation
    pub operation: AclOperation,

    /// Permission
    pub permission: AclPermission,
}

impl AclConfig {
    /// Create a new ACL configuration
    pub fn new(
        principal: impl Into<String>,
        resource_type: AclResourceType,
        resource_name: impl Into<String>,
        operation: AclOperation,
    ) -> Self {
        Self {
            principal: principal.into(),
            resource_type,
            resource_name: resource_name.into(),
            operation,
            permission: AclPermission::Allow,
        }
    }
}

/// Get standard ACL configurations for LLM Analytics
pub fn get_standard_acls() -> Vec<AclConfig> {
    let mut acls = Vec::new();

    // Producer ACLs
    let producer = "llm-analytics-producer";
    let topics = vec![
        "llm-events", "llm-metrics", "llm-analytics", "llm-traces", "llm-errors",
        "llm-audit", "llm-alerts", "llm-session-events", "llm-user-feedback",
        "llm-cost-tracking",
    ];

    for topic in topics {
        acls.push(AclConfig::new(producer, AclResourceType::Topic, topic, AclOperation::Write));
    }
    acls.push(AclConfig::new(producer, AclResourceType::Topic, "llm-*", AclOperation::Describe));
    acls.push(AclConfig::new(producer, AclResourceType::Cluster, "kafka-cluster", AclOperation::Create));

    // Consumer ACLs
    let consumer = "llm-analytics-consumer";
    let consumer_topics = vec![
        "llm-events", "llm-metrics", "llm-analytics", "llm-traces", "llm-errors",
        "llm-audit", "llm-alerts", "llm-session-events", "llm-user-feedback",
        "llm-cost-tracking", "llm-aggregated-metrics", "llm-model-performance",
        "llm-usage-stats",
    ];

    for topic in consumer_topics {
        acls.push(AclConfig::new(consumer, AclResourceType::Topic, topic, AclOperation::Read));
    }
    acls.push(AclConfig::new(consumer, AclResourceType::Topic, "llm-*", AclOperation::Describe));

    // Consumer groups
    let groups = vec!["llm-analytics-group", "llm-stream-processor"];
    for group in groups {
        acls.push(AclConfig::new(consumer, AclResourceType::Group, group, AclOperation::Read));
        acls.push(AclConfig::new(consumer, AclResourceType::Group, group, AclOperation::Describe));
    }

    acls
}

/// Cluster health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterHealth {
    /// Number of brokers
    pub broker_count: usize,

    /// Number of topics
    pub topic_count: usize,

    /// Number of LLM Analytics topics
    pub llm_topic_count: usize,

    /// Under-replicated partitions
    pub under_replicated_partitions: usize,

    /// Offline partitions
    pub offline_partitions: usize,

    /// Overall health status
    pub healthy: bool,

    /// Health messages
    pub messages: Vec<String>,
}

impl ClusterHealth {
    /// Create a new cluster health status
    pub fn new() -> Self {
        Self {
            broker_count: 0,
            topic_count: 0,
            llm_topic_count: 0,
            under_replicated_partitions: 0,
            offline_partitions: 0,
            healthy: true,
            messages: Vec::new(),
        }
    }

    /// Add a health message
    pub fn add_message(&mut self, message: impl Into<String>) {
        self.messages.push(message.into());
    }

    /// Mark as unhealthy
    pub fn mark_unhealthy(&mut self, reason: impl Into<String>) {
        self.healthy = false;
        self.messages.push(format!("ERROR: {}", reason.into()));
    }
}

impl Default for ClusterHealth {
    fn default() -> Self {
        Self::new()
    }
}
