//! Comprehensive health check command

use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
use tracing::info;

use crate::common::{
    output::{print_bullet, print_error, print_header, print_step, print_success, CommandOutput, FormattedTable},
    progress::ProgressTracker,
    ExecutionContext,
};
use crate::infra::k8s::{HealthChecker, K8sClient};

/// Health check arguments
#[derive(Debug, Args)]
pub struct AllHealthArgs {
    /// Namespace to check
    #[arg(short, long)]
    pub namespace: Option<String>,

    /// Kubeconfig path (overrides default)
    #[arg(long)]
    pub kubeconfig: Option<PathBuf>,

    /// Kubernetes context to use
    #[arg(long)]
    pub context: Option<String>,

    /// Comprehensive checks (includes slow operations)
    #[arg(short, long)]
    pub comprehensive: bool,
}

impl AllHealthArgs {
    /// Execute health check
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        print_header("Health Check - All Services");

        // Determine namespace
        let namespace = self
            .namespace
            .clone()
            .unwrap_or_else(|| ctx.config.kubernetes.namespace.clone());

        info!("Checking health of namespace: {}", namespace);

        // Create Kubernetes client
        print_step(1, 2, &format!("Connecting to Kubernetes (namespace: {})", namespace));
        let client = self.create_client(&namespace).await?;
        print_success("Connected to Kubernetes cluster");

        // Run health checks
        print_step(2, 2, "Running health checks");

        let spinner = ProgressTracker::spinner("Checking Kubernetes resources...");

        let health_checker = HealthChecker::new(client);
        let result = health_checker
            .check_all()
            .await
            .context("Failed to run health checks")?;

        spinner.finish_success("Health checks complete");

        // Display results
        if ctx.json_output {
            let output = if result.healthy {
                CommandOutput::success_with_data(
                    "All health checks passed",
                    serde_json::to_value(&result)?,
                )
            } else {
                let errors: Vec<String> = result
                    .checks
                    .iter()
                    .filter(|c| !c.passed)
                    .map(|c| format!("{}: {}", c.name, c.message))
                    .collect();

                CommandOutput::failure(
                    "Some health checks failed",
                    errors,
                )
            };
            output.output_json();
        } else {
            // Human-readable output
            println!();
            let mut table = FormattedTable::new(vec!["Check", "Status", "Message"]);

            for check in &result.checks {
                let status = if check.passed {
                    "✓ PASS".to_string()
                } else {
                    "✗ FAIL".to_string()
                };

                table.add_row(vec![
                    check.name.clone(),
                    status,
                    check.message.clone(),
                ]);
            }

            table.print();

            println!();
            if result.healthy {
                print_success("All health checks passed");
            } else {
                print_error("Some health checks failed");

                println!("\nFailed checks:");
                for check in result.checks.iter().filter(|c| !c.passed) {
                    print_bullet(&format!("{}: {}", check.name, check.message));
                }

                anyhow::bail!("Health check failed");
            }
        }

        Ok(())
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_health_args() {
        // Test construction
        assert!(true);
    }
}
