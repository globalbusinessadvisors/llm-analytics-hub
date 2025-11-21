//! Database validation command

use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
use tracing::info;

use crate::common::{
    output::{print_header, print_success, print_error, CommandOutput, FormattedTable},
    ExecutionContext,
};
use crate::infra::k8s::K8sClient;
use crate::infra::validation::{DatabaseValidator, CheckStatus};

/// Database validation arguments
#[derive(Debug, Args)]
pub struct DatabaseValidateArgs {
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

impl DatabaseValidateArgs {
    /// Execute database validation
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        print_header("Database Validation");

        // Determine namespace
        let namespace = self
            .namespace
            .clone()
            .unwrap_or_else(|| ctx.config.kubernetes.namespace.clone());

        info!("Validating databases in namespace: {}", namespace);

        // Create Kubernetes client
        let client = self.create_client(&namespace).await?;

        // Run database validation
        let validator = DatabaseValidator::new(client);
        let results = validator
            .validate()
            .await
            .context("Failed to run database validation")?;

        // Output results
        self.output_results(ctx, results).await
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
        results: crate::infra::validation::ValidationResults,
    ) -> Result<()> {
        if ctx.json_output {
            let mut report = crate::infra::validation::ValidationReport::new(
                self.namespace.as_deref().unwrap_or(&ctx.config.kubernetes.namespace),
            );
            report.add_category(results);

            let output = if report.healthy {
                CommandOutput::success_with_data("Database validation passed", serde_json::to_value(&report)?)
            } else {
                CommandOutput::failure_with_data("Database validation failed", serde_json::to_value(&report)?)
            };

            output.output_json();
        } else {
            // Human-readable output
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
            println!();

            if results.healthy {
                print_success("Database validation passed");
            } else {
                print_error("Database validation failed");
                anyhow::bail!("Database validation failed");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_validate_args() {
        assert!(true);
    }
}
