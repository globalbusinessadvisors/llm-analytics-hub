//! Redis operation commands

pub mod init;
pub mod verify;

use anyhow::Result;
use clap::Subcommand;

use crate::common::ExecutionContext;

/// Redis command
#[derive(Debug, Subcommand)]
pub enum RedisCommand {
    /// Initialize Redis cluster
    Init(init::RedisInitArgs),

    /// Verify Redis cluster health
    Verify(verify::RedisVerifyArgs),
}

impl RedisCommand {
    /// Execute the Redis command
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        match self {
            RedisCommand::Init(args) => args.execute(ctx).await,
            RedisCommand::Verify(args) => args.execute(ctx).await,
        }
    }
}
