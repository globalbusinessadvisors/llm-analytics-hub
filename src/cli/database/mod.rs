//! Database operation commands

pub mod backup;
pub mod init;
pub mod restore;

use anyhow::Result;
use clap::Subcommand;

use crate::common::ExecutionContext;

/// Database command
#[derive(Debug, Subcommand)]
pub enum DatabaseCommand {
    /// Initialize databases
    Init(init::DatabaseInitArgs),

    /// Run migrations (delegates to existing db-migrate tool)
    #[command(hide = true)]
    Migrate,

    /// Backup databases
    Backup(backup::BackupArgs),

    /// List available backups
    ListBackups(backup::ListBackupsArgs),

    /// Restore from backup
    Restore(restore::RestoreArgs),

    /// Verify backup integrity
    VerifyBackup(restore::VerifyBackupArgs),

    /// Validate database health (future implementation)
    #[command(hide = true)]
    Validate,

    /// Connect to database shell (future implementation)
    #[command(hide = true)]
    Connect,
}

impl DatabaseCommand {
    /// Execute the database command
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        match self {
            DatabaseCommand::Init(args) => args.execute(ctx).await,
            DatabaseCommand::Migrate => {
                anyhow::bail!("Use the standalone 'db-migrate' tool for migrations")
            }
            DatabaseCommand::Backup(args) => args.execute(ctx).await,
            DatabaseCommand::ListBackups(args) => args.execute(ctx).await,
            DatabaseCommand::Restore(args) => args.execute(ctx).await,
            DatabaseCommand::VerifyBackup(args) => args.execute(ctx).await,
            DatabaseCommand::Validate => {
                anyhow::bail!("Database validation not yet implemented")
            }
            DatabaseCommand::Connect => {
                anyhow::bail!("Database connect not yet implemented")
            }
        }
    }
}
