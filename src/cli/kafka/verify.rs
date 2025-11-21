//! Kafka cluster verification command

use anyhow::{Context, Result};
use clap::Args;
use tracing::info;

use crate::common::{
    output::{print_header, print_success, print_error, CommandOutput, FormattedTable},
    ExecutionContext,
};
use crate::infra::kafka::ClusterVerifier;

/// Kafka verification arguments
#[derive(Debug, Args)]
pub struct KafkaVerifyArgs {
    /// Kafka bootstrap servers
    #[arg(long, env = "KAFKA_BOOTSTRAP_SERVERS", default_value = "kafka:9092")]
    pub bootstrap_servers: String,
}

impl KafkaVerifyArgs {
    /// Execute cluster verification
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        print_header("Kafka Cluster Verification");

        info!("Verifying Kafka cluster: {}", self.bootstrap_servers);

        if ctx.dry_run {
            println!("[DRY RUN] Would verify cluster at {}", self.bootstrap_servers);
            return Ok(());
        }

        let verifier = ClusterVerifier::new(&self.bootstrap_servers)
            .context("Failed to create cluster verifier")?;

        let health = verifier
            .verify()
            .await
            .context("Failed to verify cluster")?;

        if ctx.json_output {
            let output = if health.healthy {
                CommandOutput::success_with_data("Cluster is healthy", serde_json::to_value(&health)?)
            } else {
                CommandOutput::failure_with_data("Cluster has issues", serde_json::to_value(&health)?)
            };
            output.output_json();
        } else {
            // Human-readable output
            println!("\n=== Cluster Status ===\n");

            let mut table = FormattedTable::new(vec!["Metric", "Value"]);
            table.add_row(vec!["Brokers".to_string(), health.broker_count.to_string()]);
            table.add_row(vec!["Topics".to_string(), health.topic_count.to_string()]);
            table.add_row(vec!["LLM Topics".to_string(), health.llm_topic_count.to_string()]);
            table.add_row(vec![
                "Under-Replicated Partitions".to_string(),
                health.under_replicated_partitions.to_string(),
            ]);
            table.add_row(vec![
                "Offline Partitions".to_string(),
                health.offline_partitions.to_string(),
            ]);
            table.print();

            println!("\n=== Health Messages ===\n");
            for msg in &health.messages {
                if msg.starts_with("ERROR:") {
                    println!("✗ {}", msg);
                } else if msg.starts_with("WARNING:") {
                    println!("⚠ {}", msg);
                } else {
                    println!("✓ {}", msg);
                }
            }

            println!();
            if health.healthy {
                print_success("Cluster verification passed");
            } else {
                print_error("Cluster verification failed");
                anyhow::bail!("Cluster is unhealthy");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_args() {
        assert!(true);
    }
}
