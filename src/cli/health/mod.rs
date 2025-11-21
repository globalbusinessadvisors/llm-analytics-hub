//! Health check commands

pub mod all;

use anyhow::Result;
use clap::Subcommand;

use crate::common::ExecutionContext;

/// Health check command
#[derive(Debug, Subcommand)]
pub enum HealthCommand {
    /// Check all services
    All(all::AllHealthArgs),

    /// Check API service (future implementation)
    #[command(hide = true)]
    Api,

    /// Check databases (future implementation)
    #[command(hide = true)]
    Databases,

    /// Check Kafka (future implementation)
    #[command(hide = true)]
    Kafka,

    /// Check Redis (future implementation)
    #[command(hide = true)]
    Redis,
}

impl HealthCommand {
    /// Execute the health check command
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        match self {
            HealthCommand::All(args) => args.execute(ctx).await,
            HealthCommand::Api => {
                anyhow::bail!("API health check not yet implemented")
            }
            HealthCommand::Databases => {
                anyhow::bail!("Database health check not yet implemented")
            }
            HealthCommand::Kafka => {
                anyhow::bail!("Kafka health check not yet implemented")
            }
            HealthCommand::Redis => {
                anyhow::bail!("Redis health check not yet implemented")
            }
        }
    }
}
