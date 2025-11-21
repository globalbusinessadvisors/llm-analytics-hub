//! Cloud provider abstractions
//!
//! This module provides unified abstractions for deploying to multiple cloud providers:
//! - AWS (EKS, RDS, ElastiCache, MSK)
//! - GCP (GKE, Cloud SQL, Memorystore, Pub/Sub)
//! - Azure (AKS, PostgreSQL, Redis, Event Hubs)

pub mod aws;
pub mod gcp;
pub mod azure;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Cloud provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloudProvider {
    AWS,
    GCP,
    Azure,
}

/// Cloud deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudDeploymentConfig {
    /// Cloud provider
    pub provider: CloudProvider,

    /// Environment (dev, staging, production)
    pub environment: String,

    /// Region
    pub region: String,

    /// Cluster name
    pub cluster_name: String,

    /// Enable database deployment
    pub deploy_databases: bool,

    /// Enable monitoring
    pub enable_monitoring: bool,

    /// Tags for resources
    pub tags: std::collections::HashMap<String, String>,
}

impl CloudDeploymentConfig {
    /// Create a new cloud deployment configuration
    pub fn new(
        provider: CloudProvider,
        environment: impl Into<String>,
        region: impl Into<String>,
    ) -> Self {
        let environment = environment.into();
        let region_str = region.into();

        Self {
            provider,
            environment: environment.clone(),
            region: region_str.clone(),
            cluster_name: format!("llm-analytics-{}", environment),
            deploy_databases: true,
            enable_monitoring: true,
            tags: std::collections::HashMap::new(),
        }
    }

    /// Add a tag
    pub fn with_tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.insert(key.into(), value.into());
        self
    }
}

/// Cloud deployment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudDeploymentResult {
    /// Success status
    pub success: bool,

    /// Cluster endpoint
    pub cluster_endpoint: Option<String>,

    /// Kubeconfig path
    pub kubeconfig_path: Option<String>,

    /// Deployed resources
    pub resources: Vec<DeployedResource>,

    /// Messages
    pub messages: Vec<String>,
}

/// Deployed resource information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployedResource {
    pub resource_type: String,
    pub name: String,
    pub id: String,
    pub endpoint: Option<String>,
}

/// Cloud provider trait
#[async_trait]
pub trait CloudProviderOps: Send + Sync {
    /// Deploy infrastructure
    async fn deploy(&self, config: &CloudDeploymentConfig) -> Result<CloudDeploymentResult>;

    /// Destroy infrastructure
    async fn destroy(&self, config: &CloudDeploymentConfig) -> Result<()>;

    /// Get deployment status
    async fn status(&self, config: &CloudDeploymentConfig) -> Result<CloudDeploymentResult>;

    /// Validate prerequisites
    async fn validate_prerequisites(&self) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_deployment_config() {
        let config = CloudDeploymentConfig::new(CloudProvider::AWS, "dev", "us-east-1")
            .with_tag("team", "platform")
            .with_tag("project", "llm-analytics");

        assert_eq!(config.provider, CloudProvider::AWS);
        assert_eq!(config.environment, "dev");
        assert_eq!(config.region, "us-east-1");
        assert_eq!(config.tags.len(), 2);
    }
}
