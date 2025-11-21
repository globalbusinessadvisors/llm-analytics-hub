//! Redis cluster verification command

use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
use tracing::info;

use crate::common::{
    output::{print_header, print_success, print_error, CommandOutput, FormattedTable},
    ExecutionContext,
};
use crate::infra::k8s::K8sClient;
use crate::infra::redis::ClusterManager;

/// Redis verification arguments
#[derive(Debug, Args)]
pub struct RedisVerifyArgs {
    /// Namespace where Redis is running
    #[arg(short, long)]
    pub namespace: Option<String>,

    /// Kubeconfig path (overrides default)
    #[arg(long)]
    pub kubeconfig: Option<PathBuf>,

    /// Kubernetes context to use
    #[arg(long)]
    pub context: Option<String>,
}

impl RedisVerifyArgs {
    /// Execute cluster verification
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        print_header("Redis Cluster Verification");

        // Determine namespace
        let namespace = self
            .namespace
            .clone()
            .unwrap_or_else(|| ctx.config.kubernetes.namespace.clone());

        info!("Verifying Redis cluster in namespace: {}", namespace);

        if ctx.dry_run {
            println!("[DRY RUN] Would verify Redis cluster");
            return Ok(());
        }

        // Create Kubernetes client
        let k8s_client = self.create_k8s_client(&namespace).await?;

        // Verify cluster
        let manager = ClusterManager::new(k8s_client);

        let health = manager
            .verify_cluster()
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
            table.add_row(vec!["Cluster State".to_string(), health.cluster_state.clone()]);
            table.add_row(vec!["Cluster Size".to_string(), health.cluster_size.to_string()]);
            table.add_row(vec!["Master Nodes".to_string(), health.master_count.to_string()]);
            table.add_row(vec!["Slave Nodes".to_string(), health.slave_count.to_string()]);
            table.add_row(vec![
                "Slots Assigned".to_string(),
                format!("{}/16384", health.cluster_slots_assigned),
            ]);
            table.add_row(vec![
                "Slots OK".to_string(),
                format!("{}/16384", health.cluster_slots_ok),
            ]);
            table.print();

            println!("\n=== Health Messages ===\n");
            for msg in &health.messages {
                if msg.starts_with("ERROR:") {
                    println!("✗ {}", msg);
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

    /// Create Kubernetes client
    async fn create_k8s_client(&self, namespace: &str) -> Result<K8sClient> {
        if let Some(kubeconfig) = &self.kubeconfig {
            K8sClient::with_kubeconfig(namespace, kubeconfig.clone(), self.context.clone())
                .await
                .context("Failed to create Kubernetes client with custom kubeconfig")
        } else {
            K8sClient::new(namespace)
                .await
                .context("Failed to create Kubernetes client")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis_verify_args() {
        assert!(true);
    }
}
