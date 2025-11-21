//! Validation commands

pub mod all;
pub mod cluster;
pub mod databases;
pub mod services;
pub mod security;

use anyhow::Result;
use clap::Subcommand;

use crate::common::ExecutionContext;

/// Validate command
#[derive(Debug, Subcommand)]
pub enum ValidateCommand {
    /// Validate all components
    All(all::ValidateAllArgs),

    /// Validate cluster health and nodes
    Cluster(cluster::ClusterValidateArgs),

    /// Validate database connectivity
    Databases(databases::DatabaseValidateArgs),

    /// Validate service availability
    Services(services::ServiceValidateArgs),

    /// Validate security compliance
    Security(security::SecurityValidateArgs),
}

impl ValidateCommand {
    /// Execute the validate command
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        match self {
            ValidateCommand::All(args) => args.execute(ctx).await,
            ValidateCommand::Cluster(args) => args.execute(ctx).await,
            ValidateCommand::Databases(args) => args.execute(ctx).await,
            ValidateCommand::Services(args) => args.execute(ctx).await,
            ValidateCommand::Security(args) => args.execute(ctx).await,
        }
    }
}
