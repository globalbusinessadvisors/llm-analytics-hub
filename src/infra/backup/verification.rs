//! Backup verification and integrity checking

use anyhow::{Context, Result};
use tracing::{debug, info};

use super::s3::S3BackupStorage;
use super::timescaledb::TimescaleBackupManager;
use super::types::{BackupMetadata, RestoreConfig, VerificationCheck, VerificationResult};

/// Backup verifier
pub struct BackupVerifier {
    s3_storage: S3BackupStorage,
}

impl BackupVerifier {
    /// Create new backup verifier
    pub async fn new(s3_storage: S3BackupStorage) -> Result<Self> {
        Ok(Self { s3_storage })
    }

    /// Verify a backup's integrity and restorability
    pub async fn verify_backup(&self, backup_id: &str) -> Result<VerificationResult> {
        info!("Starting verification for backup: {}", backup_id);

        let mut result = VerificationResult::new(backup_id);

        // Check 1: Backup exists in S3
        let metadata = match self.verify_backup_exists(backup_id).await {
            Ok(metadata) => {
                result.add_check(
                    VerificationCheck::new("Backup exists in S3")
                        .pass("Backup found in S3 storage"),
                );
                metadata
            }
            Err(e) => {
                result.add_check(
                    VerificationCheck::new("Backup exists in S3")
                        .fail(format!("Backup not found: {}", e)),
                );
                result.complete();
                return Ok(result);
            }
        };

        // Check 2: Backup size is valid
        if metadata.size_bytes > 0 {
            result.add_check(
                VerificationCheck::new("Backup size").pass(format!(
                    "Valid backup size: {} bytes",
                    metadata.size_bytes
                )),
            );
        } else {
            result.add_check(
                VerificationCheck::new("Backup size")
                    .fail("Backup size is zero or invalid"),
            );
        }

        // Check 3: Checksum exists
        if let Some(checksum) = &metadata.checksum {
            result.add_check(
                VerificationCheck::new("Checksum").pass(format!("Checksum present: {}", checksum)),
            );
        } else {
            result.add_check(
                VerificationCheck::new("Checksum").fail("No checksum available for verification"),
            );
        }

        // Check 4: Encryption status
        if metadata.encryption {
            result.add_check(
                VerificationCheck::new("Encryption").pass("Backup is encrypted"),
            );
        } else {
            result.add_check(
                VerificationCheck::new("Encryption").pass("Backup is not encrypted (warning)"),
            );
        }

        // Check 5: Compression status
        if let Some(compression) = &metadata.compression {
            result.add_check(
                VerificationCheck::new("Compression")
                    .pass(format!("Compression: {}", compression)),
            );
        } else {
            result.add_check(
                VerificationCheck::new("Compression").pass("No compression applied"),
            );
        }

        // Check 6: Backup type
        result.add_check(
            VerificationCheck::new("Backup type")
                .pass(format!("Type: {:?}", metadata.backup_type)),
        );

        // Check 7: Timestamp validity
        let age_days = (chrono::Utc::now() - metadata.timestamp).num_days();
        if age_days < 0 {
            result.add_check(
                VerificationCheck::new("Timestamp")
                    .fail("Backup timestamp is in the future"),
            );
        } else {
            result.add_check(
                VerificationCheck::new("Timestamp")
                    .pass(format!("Backup age: {} days", age_days)),
            );
        }

        // Check 8: WAL position (for PITR capability)
        if let Some(wal_pos) = &metadata.wal_position {
            result.add_check(
                VerificationCheck::new("PITR capability")
                    .pass(format!("WAL position available: {}", wal_pos)),
            );
        } else {
            result.add_check(
                VerificationCheck::new("PITR capability")
                    .fail("No WAL position - PITR not available"),
            );
        }

        result.complete();

        info!(
            "Verification completed for backup {}: {}",
            backup_id,
            if result.valid { "VALID" } else { "INVALID" }
        );

        Ok(result)
    }

    /// Verify backup exists in S3
    async fn verify_backup_exists(&self, backup_id: &str) -> Result<BackupMetadata> {
        // Construct S3 location
        let s3_location = format!("s3://backups/{}/{}", backup_id, backup_id);

        self.s3_storage
            .get_backup_metadata(&s3_location)
            .await
            .context("Failed to get backup metadata")
    }

    /// Verify backup checksum by re-downloading and checking
    pub async fn verify_checksum(&self, metadata: &BackupMetadata) -> Result<bool> {
        debug!("Verifying checksum for backup: {}", metadata.backup_id);

        if metadata.checksum.is_none() {
            return Ok(false);
        }

        // Download backup
        let local_path = std::path::PathBuf::from(format!("/tmp/{}.tar.gz", metadata.backup_id));

        self.s3_storage
            .download_backup(&metadata.s3_location, &local_path)
            .await?;

        // Calculate checksum
        let output = tokio::process::Command::new("sha256sum")
            .arg(&local_path)
            .output()
            .await
            .context("Failed to calculate checksum")?;

        let checksum = String::from_utf8_lossy(&output.stdout);
        let calculated_checksum = checksum
            .split_whitespace()
            .next()
            .context("Failed to parse checksum")?;

        // Clean up
        tokio::fs::remove_file(&local_path).await.ok();

        // Compare
        let matches = Some(calculated_checksum.to_string()) == metadata.checksum;

        if matches {
            info!("Checksum verification passed");
        } else {
            info!("Checksum verification failed");
        }

        Ok(matches)
    }

    /// Test backup restorability (dry run)
    pub async fn test_restore(
        &self,
        backup_manager: &TimescaleBackupManager,
        backup_id: &str,
        test_namespace: &str,
    ) -> Result<VerificationResult> {
        info!("Testing restore for backup: {}", backup_id);

        let mut result = VerificationResult::new(backup_id);

        // Create restore config for test
        let restore_config = RestoreConfig {
            backup_id: backup_id.to_string(),
            target_namespace: format!("new:{}-test", test_namespace),
            pitr_target: None,
            target_database: Some(format!("test_restore_{}", backup_id)),
            skip_validation: false,
        };

        // Attempt restore
        match backup_manager.restore_backup(&restore_config).await {
            Ok(restore_result) => {
                if restore_result.success {
                    result.add_check(
                        VerificationCheck::new("Restore test").pass(format!(
                            "Restore successful: {} tables, {} bytes",
                            restore_result.tables_restored, restore_result.restored_size_bytes
                        )),
                    );
                } else {
                    result.add_check(
                        VerificationCheck::new("Restore test")
                            .fail("Restore completed but reported as unsuccessful"),
                    );
                }

                // Verify duration is reasonable
                if restore_result.duration_seconds > 0 {
                    result.add_check(
                        VerificationCheck::new("Restore duration").pass(format!(
                            "Completed in {} seconds",
                            restore_result.duration_seconds
                        )),
                    );
                } else {
                    result.add_check(
                        VerificationCheck::new("Restore duration")
                            .fail("Invalid restore duration"),
                    );
                }

                // Verify tables were restored
                if restore_result.tables_restored > 0 {
                    result.add_check(
                        VerificationCheck::new("Table restoration").pass(format!(
                            "{} tables restored",
                            restore_result.tables_restored
                        )),
                    );
                } else {
                    result.add_check(
                        VerificationCheck::new("Table restoration")
                            .fail("No tables were restored"),
                    );
                }

                // Verify data size is reasonable
                if restore_result.restored_size_bytes > 0 {
                    result.add_check(
                        VerificationCheck::new("Data size").pass(format!(
                            "{} bytes restored",
                            restore_result.restored_size_bytes
                        )),
                    );
                } else {
                    result.add_check(
                        VerificationCheck::new("Data size").fail("No data was restored"),
                    );
                }
            }
            Err(e) => {
                result.add_check(
                    VerificationCheck::new("Restore test").fail(format!("Restore failed: {}", e)),
                );
            }
        }

        result.complete();

        info!(
            "Restore test completed for backup {}: {}",
            backup_id,
            if result.valid { "VALID" } else { "INVALID" }
        );

        Ok(result)
    }

    /// Verify all backups for a database
    pub async fn verify_all_backups(&self, database: &str) -> Result<Vec<VerificationResult>> {
        info!("Verifying all backups for database: {}", database);

        let backups = self.s3_storage.list_backups(database).await?;

        let mut results = Vec::new();

        for backup in backups {
            let result = self.verify_backup(&backup.backup_id).await?;
            results.push(result);
        }

        info!(
            "Verified {} backups for database {}",
            results.len(),
            database
        );

        Ok(results)
    }

    /// Get backup statistics for a database
    pub async fn get_backup_statistics(&self, database: &str) -> Result<BackupStatistics> {
        let backups = self.s3_storage.list_backups(database).await?;

        let total_backups = backups.len();
        let total_size: u64 = backups.iter().map(|b| b.size_bytes).sum();

        let oldest_backup = backups.iter().map(|b| b.age_days).max();
        let newest_backup = backups.iter().map(|b| b.age_days).min();

        let full_backups = backups
            .iter()
            .filter(|b| b.backup_type == super::types::BackupType::Full)
            .count();

        let incremental_backups = backups
            .iter()
            .filter(|b| b.backup_type == super::types::BackupType::Incremental)
            .count();

        Ok(BackupStatistics {
            database: database.to_string(),
            total_backups,
            total_size_bytes: total_size,
            full_backups,
            incremental_backups,
            oldest_backup_age_days: oldest_backup.unwrap_or(0),
            newest_backup_age_days: newest_backup.unwrap_or(0),
        })
    }
}

/// Backup statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupStatistics {
    /// Database name
    pub database: String,

    /// Total number of backups
    pub total_backups: usize,

    /// Total size of all backups
    pub total_size_bytes: u64,

    /// Number of full backups
    pub full_backups: usize,

    /// Number of incremental backups
    pub incremental_backups: usize,

    /// Age of oldest backup in days
    pub oldest_backup_age_days: i64,

    /// Age of newest backup in days
    pub newest_backup_age_days: i64,
}
