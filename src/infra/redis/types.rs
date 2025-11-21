//! Redis type definitions

use serde::{Deserialize, Serialize};

/// Redis cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Number of replicas per master
    pub replicas_per_master: usize,

    /// Total number of nodes
    pub total_nodes: usize,

    /// Service name
    pub service_name: String,
}

impl Default for ClusterConfig {
    fn default() -> Self {
        Self {
            replicas_per_master: 1,
            total_nodes: 6,
            service_name: "redis-cluster".to_string(),
        }
    }
}

/// Redis cluster health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterHealth {
    /// Cluster state (ok, fail)
    pub cluster_state: String,

    /// Number of nodes
    pub cluster_size: usize,

    /// Number of master nodes
    pub master_count: usize,

    /// Number of slave nodes
    pub slave_count: usize,

    /// Cluster slots assigned
    pub cluster_slots_assigned: usize,

    /// Cluster slots ok
    pub cluster_slots_ok: usize,

    /// Overall health status
    pub healthy: bool,

    /// Health messages
    pub messages: Vec<String>,
}

impl ClusterHealth {
    /// Create a new cluster health status
    pub fn new() -> Self {
        Self {
            cluster_state: "unknown".to_string(),
            cluster_size: 0,
            master_count: 0,
            slave_count: 0,
            cluster_slots_assigned: 0,
            cluster_slots_ok: 0,
            healthy: false,
            messages: Vec::new(),
        }
    }

    /// Add a health message
    pub fn add_message(&mut self, message: impl Into<String>) {
        self.messages.push(message.into());
    }

    /// Mark as healthy
    pub fn mark_healthy(&mut self) {
        self.healthy = true;
    }

    /// Mark as unhealthy
    pub fn mark_unhealthy(&mut self, reason: impl Into<String>) {
        self.healthy = false;
        self.messages.push(format!("ERROR: {}", reason.into()));
    }
}

impl Default for ClusterHealth {
    fn default() -> Self {
        Self::new()
    }
}
