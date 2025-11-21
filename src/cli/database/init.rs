//! Database initialization command

use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
use tracing::info;

use crate::common::{
    output::{print_header, print_info, print_step, print_success, CommandOutput},
    progress::ProgressTracker,
    ExecutionContext,
};
use crate::infra::k8s::{DeploymentManager, DeploymentOptions, K8sClient};

/// Database initialization arguments
#[derive(Debug, Args)]
pub struct DatabaseInitArgs {
    /// Database to initialize (timescaledb, redis, kafka, all)
    #[arg(short, long, default_value = "all")]
    pub database: String,

    /// Namespace to deploy to
    #[arg(short, long)]
    pub namespace: Option<String>,

    /// Wait for deployment to complete
    #[arg(short, long, default_value_t = true)]
    pub wait: bool,

    /// Timeout for waiting (seconds)
    #[arg(short, long, default_value_t = 600)]
    pub timeout: u64,

    /// Kubeconfig path (overrides default)
    #[arg(long)]
    pub kubeconfig: Option<PathBuf>,

    /// Kubernetes context to use
    #[arg(long)]
    pub context: Option<String>,
}

impl DatabaseInitArgs {
    /// Execute database initialization
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        print_header(&format!("Database Initialization - {}", self.database));

        // Determine namespace
        let namespace = self
            .namespace
            .clone()
            .unwrap_or_else(|| ctx.config.kubernetes.namespace.clone());

        info!("Initializing database(s) in namespace: {}", namespace);

        // Validate database choice
        let databases = self.get_databases_to_init()?;
        print_info(&format!("Will initialize: {}", databases.join(", ")));

        // Create Kubernetes client
        print_step(1, 3, &format!("Connecting to Kubernetes (namespace: {})", namespace));
        let client = self.create_client(&namespace).await?;
        print_success("Connected to Kubernetes cluster");

        // Ensure namespace exists
        print_step(2, 3, "Ensuring namespace exists");
        ctx.execute(client.ensure_namespace(), "Create namespace if needed")
            .await?;
        print_success(&format!("Namespace '{}' ready", namespace));

        // Deploy databases
        print_step(3, 3, "Deploying database manifests");

        let deployment_manager = DeploymentManager::new(client);
        let options = DeploymentOptions {
            wait: self.wait,
            timeout: self.timeout,
            force: false,
        };

        let mut deployed_count = 0;
        for database in &databases {
            let spinner = ProgressTracker::spinner(&format!("Deploying {}...", database));

            let manifest = self.get_database_manifest(database)?;

            match ctx
                .execute(
                    deployment_manager.deploy_manifest(&manifest, &options),
                    &format!("Deploy {} database", database),
                )
                .await
            {
                Ok(_) => {
                    spinner.finish_success(&format!("{} deployed successfully", database));
                    deployed_count += 1;
                }
                Err(e) => {
                    spinner.finish_error(&format!("Failed to deploy {}: {}", database, e));
                    if !ctx.dry_run {
                        return Err(e);
                    }
                }
            }
        }

        // Output result
        if ctx.json_output {
            let output = CommandOutput::success_with_data(
                format!("Successfully initialized {} database(s)", deployed_count),
                serde_json::json!({
                    "namespace": namespace,
                    "databases": databases,
                    "deployed_count": deployed_count,
                }),
            );
            output.output_json();
        } else {
            print_success(&format!(
                "Successfully initialized {} database(s): {}",
                deployed_count,
                databases.join(", ")
            ));
        }

        Ok(())
    }

    /// Get list of databases to initialize
    fn get_databases_to_init(&self) -> Result<Vec<String>> {
        match self.database.as_str() {
            "all" => Ok(vec![
                "timescaledb".to_string(),
                "redis".to_string(),
                "kafka".to_string(),
            ]),
            "timescaledb" | "redis" | "kafka" => Ok(vec![self.database.clone()]),
            _ => anyhow::bail!(
                "Invalid database: {}. Valid options: timescaledb, redis, kafka, all",
                self.database
            ),
        }
    }

    /// Get database manifest (placeholder - would load from files in production)
    fn get_database_manifest(&self, database: &str) -> Result<String> {
        // In production, this would load actual manifests from files
        // For now, return a minimal valid manifest as a placeholder
        let manifest = match database {
            "timescaledb" => r#"
apiVersion: v1
kind: Service
metadata:
  name: timescaledb
  namespace: llm-analytics-hub
spec:
  ports:
  - port: 5432
    targetPort: 5432
  selector:
    app: timescaledb
"#,
            "redis" => r#"
apiVersion: v1
kind: Service
metadata:
  name: redis
  namespace: llm-analytics-hub
spec:
  ports:
  - port: 6379
    targetPort: 6379
  selector:
    app: redis
"#,
            "kafka" => r#"
apiVersion: v1
kind: Service
metadata:
  name: kafka
  namespace: llm-analytics-hub
spec:
  ports:
  - port: 9092
    targetPort: 9092
  selector:
    app: kafka
"#,
            _ => anyhow::bail!("Unknown database: {}", database),
        };

        Ok(manifest.to_string())
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
    fn test_database_init_args_validation() {
        let args = DatabaseInitArgs {
            database: "all".to_string(),
            namespace: None,
            wait: true,
            timeout: 600,
            kubeconfig: None,
            context: None,
        };

        let databases = args.get_databases_to_init().unwrap();
        assert_eq!(databases.len(), 3);
        assert!(databases.contains(&"timescaledb".to_string()));
        assert!(databases.contains(&"redis".to_string()));
        assert!(databases.contains(&"kafka".to_string()));
    }
}
