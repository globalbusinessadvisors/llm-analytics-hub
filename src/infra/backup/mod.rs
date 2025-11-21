//! Backup and restore infrastructure
//!
//! This module provides backup and restore capabilities for:
//! - TimescaleDB database backups
//! - S3 storage integration
//! - Point-in-time recovery (PITR)
//! - Backup verification and integrity checks

pub mod types;
pub mod timescaledb;
pub mod s3;
pub mod verification;

pub use types::*;
pub use timescaledb::TimescaleBackupManager;
pub use s3::S3BackupStorage;
pub use verification::BackupVerifier;
