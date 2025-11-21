//! Integration tests for Kafka and Redis management
//!
//! These tests verify Kafka topic management and Redis cluster operations.

use llm_analytics_hub::infra::kafka::types::*;
use llm_analytics_hub::infra::redis::types::*;

// ============================================================================
// KAFKA TESTS
// ============================================================================

#[test]
fn test_kafka_topic_config_creation() {
    let config = TopicConfig::new("test-topic", 3, 2, "Test topic");

    assert_eq!(config.name, "test-topic");
    assert_eq!(config.partitions, 3);
    assert_eq!(config.replication_factor, 2);
    assert_eq!(config.description, "Test topic");
}

#[test]
fn test_kafka_topic_config_with_settings() {
    let config = TopicConfig::new("test-topic", 16, 3, "Test")
        .with_retention_ms(604_800_000)
        .with_compression("lz4")
        .with_min_isr(2);

    assert_eq!(config.partitions, 16);
    assert_eq!(config.replication_factor, 3);
    assert!(config.retention_ms.is_some());
    assert_eq!(config.retention_ms.unwrap(), 604_800_000);
    assert!(config.compression_type.is_some());
    assert_eq!(config.compression_type.unwrap(), "lz4");
    assert!(config.min_insync_replicas.is_some());
    assert_eq!(config.min_insync_replicas.unwrap(), 2);
}

#[test]
fn test_llm_topic_configs() {
    let configs = get_llm_topic_configs();

    assert_eq!(configs.len(), 14);

    // Verify first topic (llm-events)
    let llm_events = &configs[0];
    assert_eq!(llm_events.name, "llm-events");
    assert_eq!(llm_events.partitions, 32);
    assert_eq!(llm_events.replication_factor, 3);
}

#[test]
fn test_llm_topics_all_unique() {
    let configs = get_llm_topic_configs();
    let mut names: Vec<String> = configs.iter().map(|c| c.name.clone()).collect();
    names.sort();
    names.dedup();

    assert_eq!(names.len(), configs.len(), "Topic names should be unique");
}

#[test]
fn test_llm_topics_all_have_compression() {
    let configs = get_llm_topic_configs();

    for config in configs {
        assert!(
            config.compression_type.is_some(),
            "Topic {} should have compression",
            config.name
        );
        assert_eq!(
            config.compression_type.unwrap(),
            "lz4",
            "Topic {} should use lz4 compression",
            config.name
        );
    }
}

#[test]
fn test_llm_topics_all_have_min_isr() {
    let configs = get_llm_topic_configs();

    for config in configs {
        assert!(
            config.min_insync_replicas.is_some(),
            "Topic {} should have min ISR",
            config.name
        );
        assert_eq!(
            config.min_insync_replicas.unwrap(),
            2,
            "Topic {} should have min ISR of 2",
            config.name
        );
    }
}

#[test]
fn test_acl_config_creation() {
    let config = AclConfig {
        principal: "User:test-user".to_string(),
        resource_type: "Topic".to_string(),
        resource_name: "test-topic".to_string(),
        operation: "Read".to_string(),
        permission_type: "Allow".to_string(),
    };

    assert_eq!(config.principal, "User:test-user");
    assert_eq!(config.operation, "Read");
    assert_eq!(config.permission_type, "Allow");
}

#[test]
fn test_standard_producer_acls() {
    let acls = get_standard_producer_acls();

    assert!(!acls.is_empty());

    // All should be for llm-* topics
    for acl in &acls {
        assert!(
            acl.resource_name.starts_with("llm-"),
            "Producer ACL should be for llm- topics"
        );
        assert_eq!(acl.operation, "Write");
        assert_eq!(acl.permission_type, "Allow");
    }
}

#[test]
fn test_standard_consumer_acls() {
    let acls = get_standard_consumer_acls();

    assert!(!acls.is_empty());

    // Should have read permissions
    let has_read = acls.iter().any(|acl| acl.operation == "Read");
    assert!(has_read, "Consumer ACLs should include Read operations");
}

#[test]
fn test_cluster_health_creation() {
    let health = ClusterHealth {
        broker_count: 3,
        topic_count: 25,
        llm_topic_count: 14,
        under_replicated_partitions: 0,
        offline_partitions: 0,
        messages: vec!["Cluster is healthy".to_string()],
        healthy: true,
    };

    assert_eq!(health.broker_count, 3);
    assert_eq!(health.llm_topic_count, 14);
    assert!(health.healthy);
    assert_eq!(health.under_replicated_partitions, 0);
}

#[test]
fn test_cluster_health_unhealthy_state() {
    let health = ClusterHealth {
        broker_count: 3,
        topic_count: 25,
        llm_topic_count: 14,
        under_replicated_partitions: 5,
        offline_partitions: 2,
        messages: vec!["Some partitions are under-replicated".to_string()],
        healthy: false,
    };

    assert!(!health.healthy);
    assert_eq!(health.under_replicated_partitions, 5);
    assert_eq!(health.offline_partitions, 2);
}

// ============================================================================
// REDIS TESTS
// ============================================================================

#[test]
fn test_redis_cluster_config_default() {
    let config = ClusterConfig::default();

    assert_eq!(config.nodes, 6);
    assert_eq!(config.replicas_per_master, 1);
    assert_eq!(config.namespace, "llm-analytics-hub");
}

#[test]
fn test_redis_cluster_config_custom() {
    let config = ClusterConfig {
        nodes: 9,
        replicas_per_master: 2,
        namespace: "custom-namespace".to_string(),
    };

    assert_eq!(config.nodes, 9);
    assert_eq!(config.replicas_per_master, 2);
    assert_eq!(config.namespace, "custom-namespace");
}

#[test]
fn test_redis_cluster_health_creation() {
    let health = RedisClusterHealth {
        cluster_state: "ok".to_string(),
        cluster_size: 3,
        master_nodes: 3,
        slave_nodes: 3,
        slots_assigned: 16384,
        slots_ok: 16384,
        messages: vec!["Cluster is healthy".to_string()],
        healthy: true,
    };

    assert_eq!(health.cluster_state, "ok");
    assert_eq!(health.cluster_size, 3);
    assert!(health.healthy);
    assert_eq!(health.slots_assigned, 16384);
    assert_eq!(health.slots_ok, 16384);
}

#[test]
fn test_redis_cluster_health_incomplete_slots() {
    let health = RedisClusterHealth {
        cluster_state: "fail".to_string(),
        cluster_size: 3,
        master_nodes: 3,
        slave_nodes: 2,
        slots_assigned: 12000,
        slots_ok: 12000,
        messages: vec!["Not all slots assigned".to_string()],
        healthy: false,
    };

    assert_eq!(health.cluster_state, "fail");
    assert!(!health.healthy);
    assert!(health.slots_assigned < 16384);
}

#[test]
fn test_redis_cluster_health_serialization() {
    let health = RedisClusterHealth {
        cluster_state: "ok".to_string(),
        cluster_size: 3,
        master_nodes: 3,
        slave_nodes: 3,
        slots_assigned: 16384,
        slots_ok: 16384,
        messages: vec![],
        healthy: true,
    };

    let json = serde_json::to_string(&health).unwrap();
    assert!(!json.is_empty());

    let deserialized: RedisClusterHealth = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.cluster_state, health.cluster_state);
    assert_eq!(deserialized.cluster_size, health.cluster_size);
}

#[test]
fn test_kafka_cluster_health_serialization() {
    let health = ClusterHealth {
        broker_count: 3,
        topic_count: 25,
        llm_topic_count: 14,
        under_replicated_partitions: 0,
        offline_partitions: 0,
        messages: vec![],
        healthy: true,
    };

    let json = serde_json::to_string_pretty(&health).unwrap();
    assert!(!json.is_empty());
    assert!(json.contains("\"broker_count\": 3"));
    assert!(json.contains("\"healthy\": true"));
}

#[test]
fn test_topic_config_serialization() {
    let config = TopicConfig::new("test", 16, 3, "Test topic")
        .with_compression("lz4");

    let json = serde_json::to_string(&config).unwrap();
    assert!(!json.is_empty());

    let deserialized: TopicConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, config.name);
    assert_eq!(deserialized.partitions, config.partitions);
}

#[test]
fn test_acl_config_serialization() {
    let config = AclConfig {
        principal: "User:test".to_string(),
        resource_type: "Topic".to_string(),
        resource_name: "test-topic".to_string(),
        operation: "Read".to_string(),
        permission_type: "Allow".to_string(),
    };

    let json = serde_json::to_string(&config).unwrap();
    let deserialized: AclConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.principal, config.principal);
    assert_eq!(deserialized.operation, config.operation);
}
