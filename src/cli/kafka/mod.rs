//! Kafka operation commands

pub mod topics;
pub mod verify;
pub mod acls;

use anyhow::Result;
use clap::Subcommand;

use crate::common::ExecutionContext;

/// Kafka command
#[derive(Debug, Subcommand)]
pub enum KafkaCommand {
    /// Manage Kafka topics (create, list, describe, delete)
    Topics(topics::KafkaTopicsArgs),

    /// Verify Kafka cluster health
    Verify(verify::KafkaVerifyArgs),

    /// Manage Kafka ACLs (access control lists)
    Acls(acls::KafkaAclsArgs),
}

impl KafkaCommand {
    /// Execute the Kafka command
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        match self {
            KafkaCommand::Topics(args) => args.execute(ctx).await,
            KafkaCommand::Verify(args) => args.execute(ctx).await,
            KafkaCommand::Acls(args) => args.execute(ctx).await,
        }
    }
}
