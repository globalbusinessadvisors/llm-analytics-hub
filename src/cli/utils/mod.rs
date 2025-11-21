//! Utility CLI commands
//!
//! This module provides utility commands for operational tasks:
//! - scale: Scale deployments up/down
//! - cleanup: Safe infrastructure teardown
//! - connect: Interactive connection helpers

pub mod cleanup;
pub mod connect;
pub mod scale;

use anyhow::Result;
use clap::Subcommand;

use crate::common::ExecutionContext;

/// Utils command
#[derive(Debug, Subcommand)]
pub enum UtilsCommand {
    /// Scale deployments
    Scale(scale::ScaleArgs),

    /// Cleanup/destroy infrastructure
    Cleanup(cleanup::CleanupArgs),

    /// Connect to database interactively
    Connect(connect::ConnectArgs),
}

impl UtilsCommand {
    /// Execute the utils command
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        match self {
            UtilsCommand::Scale(args) => args.execute(ctx).await,
            UtilsCommand::Cleanup(args) => args.execute(ctx).await,
            UtilsCommand::Connect(args) => args.execute(ctx).await,
        }
    }
}
