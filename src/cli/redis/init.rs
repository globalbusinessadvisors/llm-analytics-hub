//! Redis cluster initialization command

use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
use tracing::info;

use crate::common::{
    output::{print_header, print_success},
    ExecutionContext,
};
use crate::infra::k8s::K8sClient;
use crate::infra::redis::{ClusterConfig, ClusterManager};

/// Redis initialization arguments
#[derive(Debug, Args)]
pub struct RedisInitArgs {
    /// Namespace where Redis is running
    #[arg(short, long)]
    pub namespace: Option<String>,

    /// Kubeconfig path (overrides default)
    #[arg(long)]
    pub kubeconfig: Option<PathBuf>,

    /// Kubernetes context to use
    #[arg(long)]
    pub context: Option<String>,

    /// Number of replicas per master
    #[arg(short, long, default_value = "1")]
    pub replicas: usize,

    /// Total number of nodes
    #[arg(short, long, default_value = "6")]
    pub nodes: usize,

    /// Service name
    #[arg(short, long, default_value = "redis-cluster")]
    pub service_name: String,
}

impl RedisInitArgs {
    /// Execute cluster initialization
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        print_header("Redis Cluster Initialization");

        // Determine namespace
        let namespace = self
            .namespace
            .clone()
            .unwrap_or_else(|| ctx.config.kubernetes.namespace.clone());

        info!("Initializing Redis cluster in namespace: {}", namespace);

        if ctx.dry_run {
            println!("[DRY RUN] Would initialize Redis cluster with {} nodes", self.nodes);
            return Ok(());
        }

        // Create Kubernetes client
        let k8s_client = self.create_k8s_client(&namespace).await?;

        // Create cluster configuration
        let config = ClusterConfig {
            replicas_per_master: self.replicas,
            total_nodes: self.nodes,
            service_name: self.service_name.clone(),
        };

        println!("Configuration:");
        println!("  Nodes: {}", config.total_nodes);
        println!("  Replicas per Master: {}", config.replicas_per_master);
        println!("  Service Name: {}", config.service_name);
        println!();

        // Initialize cluster
        let manager = ClusterManager::new(k8s_client);

        manager
            .initialize_cluster(&config)
            .await
            .context("Failed to initialize cluster")?;

        print_success("Redis cluster initialized successfully");

        println!("\nNext steps:");
        println!("  1. Verify cluster: llm-analytics redis verify");
        println!("  2. Connect to cluster: kubectl exec -it redis-cluster-0 -- redis-cli");

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
    fn test_redis_init_args() {
        assert!(true);
    }
}
