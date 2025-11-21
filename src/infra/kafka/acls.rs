//! Kafka ACL management

use anyhow::{Context, Result};
use tracing::{debug, info};

use super::types::{AclConfig, AclOperation, AclResourceType};
use crate::infra::k8s::K8sClient;

/// ACL manager for Kafka
pub struct AclManager {
    k8s_client: K8sClient,
    bootstrap_servers: String,
}

impl AclManager {
    /// Create a new ACL manager
    pub fn new(k8s_client: K8sClient, bootstrap_servers: impl Into<String>) -> Self {
        Self {
            k8s_client,
            bootstrap_servers: bootstrap_servers.into(),
        }
    }

    /// Create ACLs from configurations
    pub async fn create_acls(&self, acls: &[AclConfig]) -> Result<Vec<String>> {
        info!("Creating {} ACLs", acls.len());

        let mut created_acls = Vec::new();

        for acl in acls {
            match self.create_acl(acl).await {
                Ok(_) => {
                    info!(
                        "✓ Created ACL: {} -> {} {} {}",
                        acl.principal,
                        self.resource_type_str(&acl.resource_type),
                        acl.resource_name,
                        self.operation_str(&acl.operation)
                    );
                    created_acls.push(format!(
                        "{}:{}:{}",
                        acl.principal,
                        acl.resource_name,
                        self.operation_str(&acl.operation)
                    ));
                }
                Err(e) => {
                    // Don't fail on duplicate ACLs
                    if e.to_string().contains("already exists") {
                        debug!("ACL already exists: {}", e);
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Ok(created_acls)
    }

    /// Create a single ACL
    pub async fn create_acl(&self, acl: &AclConfig) -> Result<()> {
        debug!("Creating ACL for {}", acl.principal);

        // Find a Kafka pod to execute the command
        let kafka_pod = self.get_kafka_pod().await?;

        let resource_flag = match acl.resource_type {
            AclResourceType::Topic => format!("--topic {}", acl.resource_name),
            AclResourceType::Group => format!("--group {}", acl.resource_name),
            AclResourceType::Cluster => format!("--cluster"),
        };

        let operation = self.operation_str(&acl.operation);

        let command = format!(
            "kafka-acls.sh --bootstrap-server {} --add --allow-principal User:{} --operation {} {} --force",
            self.bootstrap_servers,
            acl.principal,
            operation,
            resource_flag
        );

        self.k8s_client
            .exec_in_pod(&kafka_pod, &command)
            .await
            .context("Failed to create ACL")?;

        Ok(())
    }

    /// List all ACLs
    pub async fn list_acls(&self) -> Result<String> {
        debug!("Listing ACLs");

        let kafka_pod = self.get_kafka_pod().await?;

        let command = format!(
            "kafka-acls.sh --bootstrap-server {} --list",
            self.bootstrap_servers
        );

        let output = self
            .k8s_client
            .exec_in_pod(&kafka_pod, &command)
            .await
            .context("Failed to list ACLs")?;

        Ok(output)
    }

    /// Get a running Kafka pod
    async fn get_kafka_pod(&self) -> Result<String> {
        let pods = self.k8s_client.list_pods().await?;

        for pod in pods {
            if let Some(name) = &pod.metadata.name {
                if name.starts_with("kafka-") {
                    if let Some(status) = &pod.status {
                        if status.phase.as_deref() == Some("Running") {
                            return Ok(name.clone());
                        }
                    }
                }
            }
        }

        anyhow::bail!("No running Kafka pod found in namespace {}", self.k8s_client.namespace())
    }

    /// Convert resource type to string
    fn resource_type_str(&self, rt: &AclResourceType) -> &str {
        match rt {
            AclResourceType::Topic => "topic",
            AclResourceType::Group => "group",
            AclResourceType::Cluster => "cluster",
        }
    }

    /// Convert operation to string
    fn operation_str(&self, op: &AclOperation) -> &str {
        match op {
            AclOperation::Read => "READ",
            AclOperation::Write => "WRITE",
            AclOperation::Describe => "DESCRIBE",
            AclOperation::Create => "CREATE",
            AclOperation::Delete => "DELETE",
            AclOperation::All => "ALL",
        }
    }
}
