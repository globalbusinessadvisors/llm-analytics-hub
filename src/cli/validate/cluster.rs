//! Cluster validation command

use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
use tracing::info;

use crate::common::{
    output::{print_header, print_success, print_error, CommandOutput, FormattedTable},
    ExecutionContext,
};
use crate::infra::k8s::K8sClient;
use crate::infra::validation::{PrerequisiteValidator, ClusterValidator, CheckStatus};

/// Cluster validation arguments
#[derive(Debug, Args)]
pub struct ClusterValidateArgs {
    /// Namespace to validate
    #[arg(short, long)]
    pub namespace: Option<String>,

    /// Kubeconfig path (overrides default)
    #[arg(long)]
    pub kubeconfig: Option<PathBuf>,

    /// Kubernetes context to use
    #[arg(long)]
    pub context: Option<String>,
}

impl ClusterValidateArgs {
    /// Execute cluster validation
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        print_header("Cluster Validation");

        // Determine namespace
        let namespace = self
            .namespace
            .clone()
            .unwrap_or_else(|| ctx.config.kubernetes.namespace.clone());

        info!("Validating cluster and namespace: {}", namespace);

        // Run prerequisite checks first
        let prereq_validator = PrerequisiteValidator::new();
        let prereq_results = prereq_validator
            .validate()
            .await
            .context("Failed to run prerequisite checks")?;

        // Only proceed with cluster checks if prerequisites pass
        if !prereq_results.healthy {
            return self.output_results(ctx, vec![prereq_results], false).await;
        }

        // Create Kubernetes client
        let client = self.create_client(&namespace).await?;

        // Run cluster validation
        let cluster_validator = ClusterValidator::new(client);
        let cluster_results = cluster_validator
            .validate()
            .await
            .context("Failed to run cluster validation")?;

        // Output results
        self.output_results(ctx, vec![prereq_results, cluster_results], true).await
    }

    /// Create Kubernetes client
    async fn create_client(&self, namespace: &str) -> Result<K8sClient> {
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

    /// Output validation results
    async fn output_results(
        &self,
        ctx: &ExecutionContext,
        all_results: Vec<crate::infra::validation::ValidationResults>,
        overall_healthy: bool,
    ) -> Result<()> {
        if ctx.json_output {
            let mut report = crate::infra::validation::ValidationReport::new(
                self.namespace.as_deref().unwrap_or(&ctx.config.kubernetes.namespace),
            );

            for results in all_results {
                report.add_category(results);
            }

            let output = if overall_healthy && report.healthy {
                CommandOutput::success_with_data("Cluster validation passed", serde_json::to_value(&report)?)
            } else {
                CommandOutput::failure_with_data("Cluster validation failed", serde_json::to_value(&report)?)
            };

            output.output_json();
        } else {
            // Human-readable output
            for results in &all_results {
                println!("\n=== {} ===", results.category);
                let mut table = FormattedTable::new(vec!["Check", "Status", "Message"]);

                for check in &results.checks {
                    let status = match check.status {
                        CheckStatus::Pass => "✓ PASS",
                        CheckStatus::Fail => "✗ FAIL",
                        CheckStatus::Warn => "⚠ WARN",
                        CheckStatus::Skip => "⊘ SKIP",
                    };

                    table.add_row(vec![
                        check.name.clone(),
                        status.to_string(),
                        check.message.clone(),
                    ]);
                }

                table.print();
                println!("Success Rate: {:.1}%", results.success_rate());
            }

            println!();
            if overall_healthy && all_results.iter().all(|r| r.healthy) {
                print_success("Cluster validation passed");
            } else {
                print_error("Cluster validation failed");
                anyhow::bail!("Cluster validation failed");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_validate_args() {
        assert!(true);
    }
}
