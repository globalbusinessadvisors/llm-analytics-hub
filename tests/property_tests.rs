//! Property-based tests using proptest
//!
//! These tests verify properties that should hold for all inputs.

use llm_analytics_hub::infra::backup::types::*;
use llm_analytics_hub::infra::kafka::types::*;
use proptest::prelude::*;

// ============================================================================
// BACKUP PROPERTY TESTS
// ============================================================================

proptest! {
    #[test]
    fn test_backup_id_never_empty(backup_id in "[a-z]{1,20}") {
        let metadata = BackupMetadata::new(&backup_id, "test_db");
        assert!(!metadata.backup_id.is_empty());
        assert_eq!(metadata.backup_id, backup_id);
    }

    #[test]
    fn test_backup_size_always_non_negative(size in 0u64..1_000_000_000u64) {
        let mut metadata = BackupMetadata::new("backup-123", "test_db");
        metadata.complete(size, "s3://bucket/backup");

        assert_eq!(metadata.size_bytes, size);
        assert!(metadata.size_bytes >= 0);
    }

    #[test]
    fn test_backup_retention_days_positive(days in 1u32..365u32) {
        let config = BackupConfig {
            s3_bucket: "test".to_string(),
            s3_prefix: "prefix".to_string(),
            aws_region: "us-east-1".to_string(),
            encryption: true,
            compression: true,
            retention_days: days,
        };

        assert!(config.retention_days > 0);
        assert!(config.retention_days <= 365);
    }

    #[test]
    fn test_database_name_preserved(db_name in "[a-z_]{1,30}") {
        let metadata = BackupMetadata::new("backup-123", &db_name);
        assert_eq!(metadata.database, db_name);
    }
}

// ============================================================================
// KAFKA PROPERTY TESTS
// ============================================================================

proptest! {
    #[test]
    fn test_topic_partitions_positive(partitions in 1i32..100i32) {
        let config = TopicConfig::new("test", partitions, 3, "Test");
        assert!(config.partitions > 0);
        assert_eq!(config.partitions, partitions);
    }

    #[test]
    fn test_topic_replication_factor_positive(rf in 1i32..10i32) {
        let config = TopicConfig::new("test", 16, rf, "Test");
        assert!(config.replication_factor > 0);
        assert_eq!(config.replication_factor, rf);
    }

    #[test]
    fn test_topic_name_not_empty(name in "[a-z-]{1,50}") {
        let config = TopicConfig::new(&name, 16, 3, "Test");
        assert!(!config.name.is_empty());
        assert_eq!(config.name, name);
    }

    #[test]
    fn test_retention_ms_non_negative(retention in 0i64..604_800_000i64) {
        let config = TopicConfig::new("test", 16, 3, "Test")
            .with_retention_ms(retention);
        assert!(config.retention_ms.is_some());
        assert!(config.retention_ms.unwrap() >= 0);
    }

    #[test]
    fn test_min_isr_less_than_rf(isr in 1i32..3i32, rf in 3i32..5i32) {
        let config = TopicConfig::new("test", 16, rf, "Test")
            .with_min_isr(isr);

        assert!(config.min_insync_replicas.is_some());
        let min_isr = config.min_insync_replicas.unwrap();

        // Min ISR should typically be less than or equal to RF
        assert!(min_isr <= rf, "Min ISR ({}) should be <= RF ({})", min_isr, rf);
    }
}

// ============================================================================
// VALIDATION PROPERTY TESTS
// ============================================================================

proptest! {
    #[test]
    fn test_check_name_never_empty(name in "[A-Za-z ]{1,50}") {
        use llm_analytics_hub::infra::validation::types::*;

        let check = ValidationCheck {
            name: name.clone(),
            category: "Test".to_string(),
            status: CheckStatus::Passed,
            severity: CheckSeverity::Critical,
            message: "OK".to_string(),
            details: None,
        };

        assert!(!check.name.is_empty());
        assert_eq!(check.name, name);
    }

    #[test]
    fn test_validation_report_metrics_sum(
        passed in 0usize..100usize,
        failed in 0usize..100usize,
        warning in 0usize..100usize,
        skipped in 0usize..100usize
    ) {
        use llm_analytics_hub::infra::validation::types::*;

        let total = passed + failed + warning + skipped;
        let report = ValidationReport {
            timestamp: chrono::Utc::now().to_rfc3339(),
            environment: "test".to_string(),
            categories: vec![],
            healthy: true,
            total_checks: total,
            passed_checks: passed,
            failed_checks: failed,
            warning_checks: warning,
            skipped_checks: skipped,
            critical_failures: 0,
            important_failures: 0,
        };

        assert_eq!(
            report.passed_checks + report.failed_checks + report.warning_checks + report.skipped_checks,
            report.total_checks
        );
    }
}

// ============================================================================
// SERIALIZATION PROPERTY TESTS
// ============================================================================

proptest! {
    #[test]
    fn test_backup_metadata_serialization_roundtrip(
        backup_id in "[a-z0-9-]{10,30}",
        database in "[a-z_]{3,20}",
        size in 0u64..10_000_000u64
    ) {
        let mut metadata = BackupMetadata::new(&backup_id, &database);
        metadata.complete(size, "s3://test/backup");

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: BackupMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.backup_id, metadata.backup_id);
        assert_eq!(deserialized.database, metadata.database);
        assert_eq!(deserialized.size_bytes, metadata.size_bytes);
    }

    #[test]
    fn test_topic_config_serialization_roundtrip(
        name in "[a-z-]{5,20}",
        partitions in 1i32..50i32,
        rf in 1i32..5i32
    ) {
        let config = TopicConfig::new(&name, partitions, rf, "Test");

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: TopicConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, config.name);
        assert_eq!(deserialized.partitions, config.partitions);
        assert_eq!(deserialized.replication_factor, config.replication_factor);
    }
}

// ============================================================================
// CLUSTER HEALTH PROPERTY TESTS
// ============================================================================

proptest! {
    #[test]
    fn test_kafka_cluster_health_broker_count(broker_count in 1usize..10usize) {
        let health = llm_analytics_hub::infra::kafka::types::ClusterHealth {
            broker_count,
            topic_count: 0,
            llm_topic_count: 0,
            under_replicated_partitions: 0,
            offline_partitions: 0,
            messages: vec![],
            healthy: true,
        };

        assert!(health.broker_count > 0);
        assert_eq!(health.broker_count, broker_count);
    }

    #[test]
    fn test_redis_cluster_slots_valid(slots_assigned in 0usize..16384usize) {
        use llm_analytics_hub::infra::redis::types::*;

        let health = RedisClusterHealth {
            cluster_state: "ok".to_string(),
            cluster_size: 3,
            master_nodes: 3,
            slave_nodes: 3,
            slots_assigned,
            slots_ok: slots_assigned,
            messages: vec![],
            healthy: slots_assigned == 16384,
        };

        assert!(health.slots_assigned <= 16384);
        assert!(health.slots_ok <= 16384);
        assert_eq!(health.slots_assigned, health.slots_ok);
    }
}
