//! Redis cluster management

use anyhow::{Context, Result};
use tracing::{debug, info};

use super::types::{ClusterConfig, ClusterHealth};
use crate::infra::k8s::K8sClient;

/// Cluster manager for Redis
pub struct ClusterManager {
    k8s_client: K8sClient,
}

impl ClusterManager {
    /// Create a new cluster manager
    pub fn new(k8s_client: K8sClient) -> Self {
        Self { k8s_client }
    }

    /// Initialize Redis cluster
    pub async fn initialize_cluster(&self, config: &ClusterConfig) -> Result<()> {
        info!("Initializing Redis cluster with {} nodes", config.total_nodes);

        // Wait for all pods to be ready
        self.wait_for_pods(config).await?;

        // Check if cluster is already initialized
        if self.is_cluster_initialized(config).await? {
            info!("Cluster is already initialized");
            return Ok(());
        }

        // Get pod IPs
        let pod_ips = self.get_pod_ips().await?;

        if pod_ips.len() != config.total_nodes {
            anyhow::bail!(
                "Expected {} pods, found {}",
                config.total_nodes,
                pod_ips.len()
            );
        }

        // Create cluster using redis-cli
        self.create_cluster(&pod_ips, config).await?;

        info!("✓ Redis cluster initialized successfully");
        Ok(())
    }

    /// Verify cluster health
    pub async fn verify_cluster(&self) -> Result<ClusterHealth> {
        info!("Verifying Redis cluster health");

        let mut health = ClusterHealth::new();

        // Get a Redis pod to execute commands
        let pod_name = self.get_redis_pod().await?;

        // Get cluster info
        let cluster_info = self.get_cluster_info(&pod_name).await?;

        // Parse cluster info
        self.parse_cluster_info(&cluster_info, &mut health);

        // Get cluster nodes
        let cluster_nodes = self.get_cluster_nodes(&pod_name).await?;

        // Parse cluster nodes
        self.parse_cluster_nodes(&cluster_nodes, &mut health);

        // Determine overall health
        if health.cluster_state == "ok" && health.cluster_slots_ok == 16384 {
            health.mark_healthy();
            health.add_message("Cluster is healthy");
        } else {
            health.mark_unhealthy("Cluster is not in healthy state");
        }

        Ok(health)
    }

    /// Wait for all pods to be ready
    async fn wait_for_pods(&self, config: &ClusterConfig) -> Result<()> {
        info!("Waiting for Redis pods to be ready...");

        let max_attempts = 60;
        for attempt in 1..=max_attempts {
            let pods = self.k8s_client.list_pods().await?;

            let redis_pods: Vec<_> = pods
                .iter()
                .filter(|p| {
                    p.metadata
                        .name
                        .as_ref()
                        .map(|n| n.starts_with(&config.service_name))
                        .unwrap_or(false)
                })
                .collect();

            let ready_pods = redis_pods
                .iter()
                .filter(|p| {
                    p.status
                        .as_ref()
                        .and_then(|s| s.phase.as_deref())
                        .unwrap_or("") == "Running"
                })
                .count();

            if ready_pods == config.total_nodes {
                info!("All {} Redis pods are ready", config.total_nodes);
                return Ok(());
            }

            if attempt % 10 == 0 {
                info!("Waiting for pods... ({}/{} ready)", ready_pods, config.total_nodes);
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }

        anyhow::bail!("Timeout waiting for Redis pods to be ready")
    }

    /// Check if cluster is already initialized
    async fn is_cluster_initialized(&self, config: &ClusterConfig) -> Result<bool> {
        let pod_name = format!("{}-0", config.service_name);

        let result = self
            .k8s_client
            .exec_in_pod(&pod_name, "redis-cli cluster info")
            .await;

        match result {
            Ok(output) => Ok(output.contains("cluster_state:ok")),
            Err(_) => Ok(false),
        }
    }

    /// Get pod IP addresses
    async fn get_pod_ips(&self) -> Result<Vec<String>> {
        let pods = self.k8s_client.list_pods().await?;

        let ips: Vec<String> = pods
            .iter()
            .filter(|p| {
                p.metadata
                    .name
                    .as_ref()
                    .map(|n| n.starts_with("redis-"))
                    .unwrap_or(false)
            })
            .filter_map(|p| {
                p.status
                    .as_ref()
                    .and_then(|s| s.pod_ip.clone())
            })
            .collect();

        Ok(ips)
    }

    /// Create Redis cluster
    async fn create_cluster(&self, pod_ips: &[String], config: &ClusterConfig) -> Result<()> {
        info!("Creating Redis cluster...");

        let pod_name = format!("{}-0", config.service_name);

        // Build cluster create command
        let nodes: Vec<String> = pod_ips.iter().map(|ip| format!("{}:6379", ip)).collect();
        let nodes_str = nodes.join(" ");

        let command = format!(
            "redis-cli --cluster create {} --cluster-replicas {} --cluster-yes",
            nodes_str, config.replicas_per_master
        );

        debug!("Executing: {}", command);

        self.k8s_client
            .exec_in_pod(&pod_name, &command)
            .await
            .context("Failed to create cluster")?;

        Ok(())
    }

    /// Get cluster info
    async fn get_cluster_info(&self, pod_name: &str) -> Result<String> {
        self.k8s_client
            .exec_in_pod(pod_name, "redis-cli cluster info")
            .await
            .context("Failed to get cluster info")
    }

    /// Get cluster nodes
    async fn get_cluster_nodes(&self, pod_name: &str) -> Result<String> {
        self.k8s_client
            .exec_in_pod(pod_name, "redis-cli cluster nodes")
            .await
            .context("Failed to get cluster nodes")
    }

    /// Parse cluster info
    fn parse_cluster_info(&self, info: &str, health: &mut ClusterHealth) {
        for line in info.lines() {
            if line.starts_with("cluster_state:") {
                health.cluster_state = line.split(':').nth(1).unwrap_or("unknown").to_string();
            } else if line.starts_with("cluster_size:") {
                if let Some(size) = line.split(':').nth(1) {
                    health.cluster_size = size.trim().parse().unwrap_or(0);
                }
            } else if line.starts_with("cluster_slots_assigned:") {
                if let Some(slots) = line.split(':').nth(1) {
                    health.cluster_slots_assigned = slots.trim().parse().unwrap_or(0);
                }
            } else if line.starts_with("cluster_slots_ok:") {
                if let Some(slots) = line.split(':').nth(1) {
                    health.cluster_slots_ok = slots.trim().parse().unwrap_or(0);
                }
            }
        }
    }

    /// Parse cluster nodes
    fn parse_cluster_nodes(&self, nodes: &str, health: &mut ClusterHealth) {
        let mut masters = 0;
        let mut slaves = 0;

        for line in nodes.lines() {
            if line.contains("master") {
                masters += 1;
            } else if line.contains("slave") {
                slaves += 1;
            }
        }

        health.master_count = masters;
        health.slave_count = slaves;
    }

    /// Get a running Redis pod
    async fn get_redis_pod(&self) -> Result<String> {
        let pods = self.k8s_client.list_pods().await?;

        for pod in pods {
            if let Some(name) = &pod.metadata.name {
                if name.starts_with("redis-") {
                    if let Some(status) = &pod.status {
                        if status.phase.as_deref() == Some("Running") {
                            return Ok(name.clone());
                        }
                    }
                }
            }
        }

        anyhow::bail!("No running Redis pod found in namespace {}", self.k8s_client.namespace())
    }
}
