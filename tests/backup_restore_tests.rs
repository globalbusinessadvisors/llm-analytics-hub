//! Integration tests for backup and restore operations
//!
//! These tests verify backup/restore functionality and S3 integration.

use llm_analytics_hub::infra::backup::types::*;
use chrono::Utc;

#[test]
fn test_backup_config_default() {
    let config = BackupConfig::default();

    assert_eq!(config.s3_bucket, "llm-analytics-backups");
    assert_eq!(config.s3_prefix, "timescaledb");
    assert_eq!(config.aws_region, "us-east-1");
    assert!(config.encryption);
    assert!(config.compression);
    assert_eq!(config.retention_days, 30);
}

#[test]
fn test_backup_config_custom() {
    let config = BackupConfig {
        s3_bucket: "my-backups".to_string(),
        s3_prefix: "postgres".to_string(),
        aws_region: "us-west-2".to_string(),
        encryption: false,
        compression: true,
        retention_days: 90,
    };

    assert_eq!(config.s3_bucket, "my-backups");
    assert_eq!(config.aws_region, "us-west-2");
    assert!(!config.encryption);
    assert_eq!(config.retention_days, 90);
}

#[test]
fn test_backup_metadata_creation() {
    let metadata = BackupMetadata::new("backup-123", "test_db");

    assert_eq!(metadata.backup_id, "backup-123");
    assert_eq!(metadata.database, "test_db");
    assert_eq!(metadata.backup_type, BackupType::Full);
    assert_eq!(metadata.status, BackupStatus::InProgress);
    assert_eq!(metadata.size_bytes, 0);
    assert!(metadata.checksum.is_none());
}

#[test]
fn test_backup_metadata_completion() {
    let mut metadata = BackupMetadata::new("backup-123", "test_db");

    metadata.complete(1024000, "s3://bucket/backup-123.tar.gz");

    assert_eq!(metadata.status, BackupStatus::Completed);
    assert_eq!(metadata.size_bytes, 1024000);
    assert_eq!(metadata.s3_location, "s3://bucket/backup-123.tar.gz");
}

#[test]
fn test_backup_metadata_failure() {
    let mut metadata = BackupMetadata::new("backup-123", "test_db");

    metadata.fail("Database connection failed");

    assert_eq!(metadata.status, BackupStatus::Failed);
    assert!(metadata.metadata.get("error").is_some());
}

#[test]
fn test_backup_type_variants() {
    let full = BackupType::Full;
    let incremental = BackupType::Incremental;
    let differential = BackupType::Differential;

    assert_ne!(full, incremental);
    assert_ne!(incremental, differential);
}

#[test]
fn test_backup_status_variants() {
    let in_progress = BackupStatus::InProgress;
    let completed = BackupStatus::Completed;
    let failed = BackupStatus::Failed;
    let verification_pending = BackupStatus::VerificationPending;
    let verified = BackupStatus::Verified;

    assert_ne!(in_progress, completed);
    assert_ne!(completed, failed);
    assert_ne!(verification_pending, verified);
}

#[test]
fn test_restore_config_creation() {
    let config = RestoreConfig {
        backup_id: "backup-123".to_string(),
        target_namespace: "default".to_string(),
        pitr_target: None,
        target_database: Some("restored_db".to_string()),
        skip_validation: false,
    };

    assert_eq!(config.backup_id, "backup-123");
    assert_eq!(config.target_namespace, "default");
    assert!(config.pitr_target.is_none());
    assert!(!config.skip_validation);
}

#[test]
fn test_restore_config_with_pitr() {
    let pitr_time = Utc::now();
    let config = RestoreConfig {
        backup_id: "backup-123".to_string(),
        target_namespace: "default".to_string(),
        pitr_target: Some(pitr_time),
        target_database: None,
        skip_validation: false,
    };

    assert!(config.pitr_target.is_some());
    assert_eq!(config.pitr_target.unwrap(), pitr_time);
}

#[test]
fn test_restore_result_creation() {
    let result = RestoreResult::new();

    assert!(!result.success);
    assert_eq!(result.duration_seconds, 0);
    assert_eq!(result.restored_size_bytes, 0);
    assert_eq!(result.tables_restored, 0);
    assert!(result.messages.is_empty());
}

#[test]
fn test_restore_result_messages() {
    let mut result = RestoreResult::new();

    result.add_message("Starting restore");
    result.add_message("Downloading backup");
    result.add_message("Extracting files");

    assert_eq!(result.messages.len(), 3);
    assert_eq!(result.messages[0], "Starting restore");
}

#[test]
fn test_verification_result_creation() {
    let result = VerificationResult::new("backup-123");

    assert_eq!(result.backup_id, "backup-123");
    assert!(result.valid);
    assert_eq!(result.status, "pending");
    assert!(result.checks.is_empty());
}

#[test]
fn test_verification_check_passed() {
    let check = VerificationCheck::new("Checksum")
        .pass("SHA256 checksum valid");

    assert!(check.passed);
    assert_eq!(check.name, "Checksum");
    assert_eq!(check.message, "SHA256 checksum valid");
}

#[test]
fn test_verification_check_failed() {
    let check = VerificationCheck::new("Size validation")
        .fail("Backup file is corrupted");

    assert!(!check.passed);
    assert_eq!(check.message, "Backup file is corrupted");
}

#[test]
fn test_verification_result_add_checks() {
    let mut result = VerificationResult::new("backup-123");

    result.add_check(VerificationCheck::new("Check 1").pass("OK"));
    result.add_check(VerificationCheck::new("Check 2").pass("OK"));

    assert_eq!(result.checks.len(), 2);
    assert!(result.valid);
}

#[test]
fn test_verification_result_with_failure() {
    let mut result = VerificationResult::new("backup-123");

    result.add_check(VerificationCheck::new("Check 1").pass("OK"));
    result.add_check(VerificationCheck::new("Check 2").fail("Failed"));

    assert_eq!(result.checks.len(), 2);
    assert!(!result.valid); // Should be invalid due to failed check
}

#[test]
fn test_verification_result_completion() {
    let mut result = VerificationResult::new("backup-123");

    result.add_check(VerificationCheck::new("Test").pass("OK"));
    result.complete();

    assert_eq!(result.status, "valid");

    let mut result2 = VerificationResult::new("backup-456");
    result2.add_check(VerificationCheck::new("Test").fail("Error"));
    result2.complete();

    assert_eq!(result2.status, "invalid");
}

#[test]
fn test_backup_entry_structure() {
    let entry = BackupEntry {
        backup_id: "backup-123".to_string(),
        timestamp: Utc::now(),
        size_bytes: 1024000,
        backup_type: BackupType::Full,
        status: BackupStatus::Completed,
        age_days: 5,
    };

    assert_eq!(entry.backup_id, "backup-123");
    assert_eq!(entry.size_bytes, 1024000);
    assert_eq!(entry.age_days, 5);
}

#[test]
fn test_backup_metadata_serialization() {
    let metadata = BackupMetadata::new("backup-123", "test_db");

    let json = serde_json::to_string(&metadata).unwrap();
    assert!(!json.is_empty());

    let deserialized: BackupMetadata = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.backup_id, metadata.backup_id);
    assert_eq!(deserialized.database, metadata.database);
}

#[test]
fn test_restore_result_serialization() {
    let result = RestoreResult {
        success: true,
        duration_seconds: 120,
        restored_size_bytes: 2048000,
        tables_restored: 42,
        messages: vec!["Restore completed".to_string()],
    };

    let json = serde_json::to_string_pretty(&result).unwrap();
    assert!(!json.is_empty());
    assert!(json.contains("\"success\": true"));
    assert!(json.contains("42"));
}
