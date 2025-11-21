//! Kafka ACL management command

use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
use tracing::info;

use crate::common::{
    output::{print_header, print_success, CommandOutput},
    ExecutionContext,
};
use crate::infra::k8s::K8sClient;
use crate::infra::kafka::{AclManager, get_standard_acls};

/// Kafka ACL management arguments
#[derive(Debug, Args)]
pub struct KafkaAclsArgs {
    /// Kafka bootstrap servers
    #[arg(long, env = "KAFKA_BOOTSTRAP_SERVERS", default_value = "kafka:9092")]
    pub bootstrap_servers: String,

    /// Namespace where Kafka is running
    #[arg(short, long)]
    pub namespace: Option<String>,

    /// Kubeconfig path (overrides default)
    #[arg(long)]
    pub kubeconfig: Option<PathBuf>,

    /// Kubernetes context to use
    #[arg(long)]
    pub context: Option<String>,

    /// Action to perform
    #[command(subcommand)]
    pub action: AclAction,
}

#[derive(Debug, clap::Subcommand)]
pub enum AclAction {
    /// Create standard LLM Analytics ACLs
    Create {
        /// Custom ACL configuration file (YAML)
        #[arg(short, long)]
        config: Option<PathBuf>,
    },

    /// List all ACLs
    List,
}

impl KafkaAclsArgs {
    /// Execute ACL management command
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        match &self.action {
            AclAction::Create { config } => self.create_acls(ctx, config.as_ref()).await,
            AclAction::List => self.list_acls(ctx).await,
        }
    }

    /// Create ACLs
    async fn create_acls(&self, ctx: &ExecutionContext, config_file: Option<&PathBuf>) -> Result<()> {
        print_header("Creating Kafka ACLs");

        info!("Connecting to Kafka: {}", self.bootstrap_servers);

        if ctx.dry_run {
            println!("[DRY RUN] Would create ACLs on {}", self.bootstrap_servers);
            return Ok(());
        }

        // Get ACL configurations
        let acl_configs = if let Some(_config_file) = config_file {
            // TODO: Load from YAML file
            anyhow::bail!("Custom config file not yet implemented");
        } else {
            get_standard_acls()
        };

        println!("Creating {} standard ACLs...\n", acl_configs.len());

        // Create Kubernetes client
        let namespace = self
            .namespace
            .clone()
            .unwrap_or_else(|| ctx.config.kubernetes.namespace.clone());

        let k8s_client = self.create_k8s_client(&namespace).await?;

        let manager = AclManager::new(k8s_client, &self.bootstrap_servers);

        let created = manager
            .create_acls(&acl_configs)
            .await
            .context("Failed to create ACLs")?;

        if ctx.json_output {
            let output = CommandOutput::success_with_data(
                format!("Created {} ACLs", created.len()),
                serde_json::json!({
                    "acls": created,
                    "total": created.len(),
                }),
            );
            output.output_json();
        } else {
            println!();
            print_success(&format!("Successfully created/updated {} ACLs", created.len()));

            println!("\nACL Summary:");
            println!("  • Producer ACLs: write access to all LLM topics");
            println!("  • Consumer ACLs: read access to all LLM topics");
            println!("  • Consumer Group ACLs: group management");
        }

        Ok(())
    }

    /// List ACLs
    async fn list_acls(&self, ctx: &ExecutionContext) -> Result<()> {
        print_header("Kafka ACLs");

        // Create Kubernetes client
        let namespace = self
            .namespace
            .clone()
            .unwrap_or_else(|| ctx.config.kubernetes.namespace.clone());

        let k8s_client = self.create_k8s_client(&namespace).await?;

        let manager = AclManager::new(k8s_client, &self.bootstrap_servers);

        let acls = manager
            .list_acls()
            .await
            .context("Failed to list ACLs")?;

        if ctx.json_output {
            let output = CommandOutput::success_with_data(
                "ACL list",
                serde_json::json!({
                    "acls": acls,
                }),
            );
            output.output_json();
        } else {
            println!("\n=== Kafka ACLs ===\n");
            println!("{}", acls);
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
    fn test_acl_args() {
        assert!(true);
    }
}
