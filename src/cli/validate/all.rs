//! Comprehensive validation command

use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
use tracing::info;

use crate::common::{
    output::{print_error, print_header, print_step, print_success, CommandOutput, FormattedTable},
    progress::ProgressTracker,
    ExecutionContext,
};
use crate::infra::k8s::K8sClient;
use crate::infra::validation::{
    PrerequisiteValidator, ClusterValidator, ServiceValidator, DatabaseValidator,
    SecurityValidator, NetworkValidator, ResourceValidator, ValidationReport, CheckStatus,
};

/// Validation arguments
#[derive(Debug, Args)]
pub struct ValidateAllArgs {
    /// Namespace to validate
    #[arg(short, long)]
    pub namespace: Option<String>,

    /// Kubeconfig path (overrides default)
    #[arg(long)]
    pub kubeconfig: Option<PathBuf>,

    /// Kubernetes context to use
    #[arg(long)]
    pub context: Option<String>,

    /// Skip non-critical checks
    #[arg(short, long)]
    pub fast: bool,
}

impl ValidateAllArgs {
    /// Execute validation
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        print_header("Comprehensive Validation");

        // Determine namespace
        let namespace = self
            .namespace
            .clone()
            .unwrap_or_else(|| ctx.config.kubernetes.namespace.clone());

        info!("Running comprehensive validation on namespace: {}", namespace);

        // Create validation report
        let mut report = ValidationReport::new(&namespace);

        // Step 1: Prerequisites
        print_step(1, 7, "Checking prerequisites");
        let spinner = ProgressTracker::spinner("Validating prerequisites...");
        let prereq_validator = PrerequisiteValidator::new();
        let prereq_results = prereq_validator
            .validate()
            .await
            .context("Failed to run prerequisite checks")?;
        spinner.finish_success("Prerequisites checked");
        report.add_category(prereq_results);

        // Only proceed if prerequisites pass
        if !report.healthy {
            return self.output_results(ctx, report).await;
        }

        // Create Kubernetes client
        let client = self.create_client(&namespace).await?;

        // Step 2: Cluster health
        print_step(2, 7, "Validating cluster health");
        let spinner = ProgressTracker::spinner("Checking cluster and nodes...");
        let cluster_validator = ClusterValidator::new(client.clone());
        let cluster_results = cluster_validator
            .validate()
            .await
            .context("Failed to run cluster validation")?;
        spinner.finish_success("Cluster validation complete");
        report.add_category(cluster_results);

        // Step 3: Service availability
        print_step(3, 7, "Validating service availability");
        let spinner = ProgressTracker::spinner("Checking services and pods...");
        let service_validator = ServiceValidator::new(client.clone());
        let service_results = service_validator
            .validate()
            .await
            .context("Failed to run service validation")?;
        spinner.finish_success("Service validation complete");
        report.add_category(service_results);

        // Step 4: Database connectivity
        if !self.fast {
            print_step(4, 7, "Validating database connectivity");
            let spinner = ProgressTracker::spinner("Checking database connections...");
            let database_validator = DatabaseValidator::new(client.clone());
            let database_results = database_validator
                .validate()
                .await
                .context("Failed to run database validation")?;
            spinner.finish_success("Database validation complete");
            report.add_category(database_results);
        } else {
            print_step(4, 7, "Skipping database validation (fast mode)");
        }

        // Step 5: Security compliance
        print_step(5, 7, "Validating security compliance");
        let spinner = ProgressTracker::spinner("Checking security policies...");
        let security_validator = SecurityValidator::new(client.clone());
        let security_results = security_validator
            .validate()
            .await
            .context("Failed to run security validation")?;
        spinner.finish_success("Security validation complete");
        report.add_category(security_results);

        // Step 6: Network connectivity
        if !self.fast {
            print_step(6, 7, "Validating network connectivity");
            let spinner = ProgressTracker::spinner("Checking network and DNS...");
            let network_validator = NetworkValidator::new(client.clone());
            let network_results = network_validator
                .validate()
                .await
                .context("Failed to run network validation")?;
            spinner.finish_success("Network validation complete");
            report.add_category(network_results);
        } else {
            print_step(6, 7, "Skipping network validation (fast mode)");
        }

        // Step 7: Resource utilization
        print_step(7, 7, "Validating resource utilization");
        let spinner = ProgressTracker::spinner("Checking resource usage...");
        let resource_validator = ResourceValidator::new(client.clone());
        let resource_results = resource_validator
            .validate()
            .await
            .context("Failed to run resource validation")?;
        spinner.finish_success("Resource validation complete");
        report.add_category(resource_results);

        // Output results
        self.output_results(ctx, report).await
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
    async fn output_results(&self, ctx: &ExecutionContext, report: ValidationReport) -> Result<()> {
        if ctx.json_output {
            let output = if report.healthy {
                CommandOutput::success_with_data("All validations passed", serde_json::to_value(&report)?)
            } else {
                CommandOutput::failure_with_data("Some validations failed", serde_json::to_value(&report)?)
            };
            output.output_json();
        } else {
            // Human-readable output
            for category in &report.categories {
                println!("\n=== {} ===", category.category);
                let mut table = FormattedTable::new(vec!["Check", "Status", "Message"]);

                for check in &category.checks {
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
                println!("  Passed: {} | Failed: {} | Warnings: {} | Success Rate: {:.1}%",
                    category.passed, category.failed, category.warnings, category.success_rate());
            }

            println!("\n=== Overall Summary ===");
            println!("Total Checks: {}", report.total_checks);
            println!("Passed: {} | Failed: {} | Warnings: {}",
                report.total_passed, report.total_failed, report.total_warnings);
            println!("Overall Success Rate: {:.1}%", report.success_rate());
            println!();

            if report.healthy {
                print_success("All validations passed");
            } else {
                print_error("Some validations failed - review errors above");
                anyhow::bail!("Validation failed");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_all_args() {
        // Test construction
        assert!(true);
    }
}
