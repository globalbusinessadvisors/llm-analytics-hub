//! Configuration management for CLI operations

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// CLI configuration loaded from files and environment
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CliConfig {
    /// Kubernetes configuration
    #[serde(default)]
    pub kubernetes: KubernetesConfig,

    /// Cloud provider configurations
    #[serde(default)]
    pub cloud: CloudConfig,

    /// Database configurations
    #[serde(default)]
    pub database: DatabaseConfig,

    /// Kafka configuration
    #[serde(default)]
    pub kafka: KafkaConfig,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            kubernetes: KubernetesConfig::default(),
            cloud: CloudConfig::default(),
            database: DatabaseConfig::default(),
            kafka: KafkaConfig::default(),
        }
    }
}

impl CliConfig {
    /// Load configuration from file and environment
    pub fn load() -> Result<Self> {
        // Try to load from config file
        let config_path = Self::config_path();

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .context("Failed to read config file")?;
            serde_yaml::from_str(&content)
                .context("Failed to parse config file")
        } else {
            // Return default config if file doesn't exist
            Ok(Self::default())
        }
    }

    /// Get config file path
    fn config_path() -> PathBuf {
        std::env::var("LLM_ANALYTICS_CONFIG")
            .ok()
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                dirs::config_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("llm-analytics")
                    .join("config.yaml")
            })
    }
}

/// Kubernetes configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KubernetesConfig {
    /// Default namespace
    pub namespace: String,

    /// Kubeconfig path (overrides default)
    pub kubeconfig: Option<PathBuf>,

    /// Context to use (overrides current context)
    pub context: Option<String>,

    /// Timeout for operations (seconds)
    pub timeout: u64,
}

impl Default for KubernetesConfig {
    fn default() -> Self {
        Self {
            namespace: "llm-analytics-hub".to_string(),
            kubeconfig: None,
            context: None,
            timeout: 300,
        }
    }
}

/// Cloud provider configurations
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CloudConfig {
    /// AWS configuration
    #[serde(default)]
    pub aws: AwsConfig,

    /// GCP configuration
    #[serde(default)]
    pub gcp: GcpConfig,

    /// Azure configuration
    #[serde(default)]
    pub azure: AzureConfig,
}

/// AWS configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AwsConfig {
    pub region: String,
    pub profile: Option<String>,
}

impl Default for AwsConfig {
    fn default() -> Self {
        Self {
            region: "us-east-1".to_string(),
            profile: None,
        }
    }
}

/// GCP configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GcpConfig {
    pub project: Option<String>,
    pub region: String,
    pub zone: String,
}

impl Default for GcpConfig {
    fn default() -> Self {
        Self {
            project: None,
            region: "us-central1".to_string(),
            zone: "us-central1-a".to_string(),
        }
    }
}

/// Azure configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AzureConfig {
    pub subscription_id: Option<String>,
    pub resource_group: Option<String>,
    pub location: String,
}

impl Default for AzureConfig {
    fn default() -> Self {
        Self {
            subscription_id: None,
            resource_group: None,
            location: "eastus".to_string(),
        }
    }
}

/// Database configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub timescaledb_url: Option<String>,
    pub redis_url: Option<String>,
    pub backup_bucket: Option<String>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            timescaledb_url: None,
            redis_url: None,
            backup_bucket: None,
        }
    }
}

/// Kafka configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KafkaConfig {
    pub bootstrap_servers: Option<String>,
    pub topics: Vec<String>,
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self {
            bootstrap_servers: None,
            topics: Vec::new(),
        }
    }
}

/// Execution context for CLI operations
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Dry-run mode (don't actually execute)
    pub dry_run: bool,

    /// Verbose output
    pub verbose: bool,

    /// JSON output format
    pub json_output: bool,

    /// Configuration
    pub config: CliConfig,
}

impl ExecutionContext {
    /// Create a new execution context
    pub fn new(dry_run: bool, verbose: bool, json_output: bool) -> Result<Self> {
        let config = CliConfig::load()?;

        Ok(Self {
            dry_run,
            verbose,
            json_output,
            config,
        })
    }

    /// Execute an operation with context
    pub async fn execute<F, T>(&self, operation: F, description: &str) -> Result<T>
    where
        F: std::future::Future<Output = Result<T>>,
    {
        if self.dry_run {
            use crate::common::output::print_dry_run;
            print_dry_run(&format!("Would execute: {}", description));
            anyhow::bail!("Dry run mode - operation not executed");
        }

        if self.verbose {
            use crate::common::output::print_info;
            print_info(&format!("Executing: {}", description));
        }

        operation.await
    }
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            dry_run: false,
            verbose: false,
            json_output: false,
            config: CliConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CliConfig::default();
        assert_eq!(config.kubernetes.namespace, "llm-analytics-hub");
        assert_eq!(config.cloud.aws.region, "us-east-1");
    }

    #[test]
    fn test_execution_context_creation() {
        let ctx = ExecutionContext::default();
        assert!(!ctx.dry_run);
        assert!(!ctx.verbose);
    }
}
