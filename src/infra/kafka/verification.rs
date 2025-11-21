//! Kafka cluster verification

use anyhow::{Context, Result};
use rdkafka::admin::AdminClient;
use rdkafka::client::DefaultClientContext;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use std::time::Duration;
use tracing::{debug, info};

use super::types::ClusterHealth;

/// Cluster verifier for Kafka
pub struct ClusterVerifier {
    bootstrap_servers: String,
    admin_client: AdminClient<DefaultClientContext>,
}

impl ClusterVerifier {
    /// Create a new cluster verifier
    pub fn new(bootstrap_servers: impl Into<String>) -> Result<Self> {
        let bootstrap_servers = bootstrap_servers.into();

        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", &bootstrap_servers)
            .set("client.id", "llm-analytics-verifier")
            .create()
            .context("Failed to create Kafka admin client")?;

        Ok(Self {
            bootstrap_servers,
            admin_client,
        })
    }

    /// Verify cluster health
    pub async fn verify(&self) -> Result<ClusterHealth> {
        info!("Verifying Kafka cluster health");

        let mut health = ClusterHealth::new();

        // 1. Check broker connectivity
        self.check_connectivity(&mut health).await?;

        // 2. Check cluster metadata
        self.check_metadata(&mut health).await?;

        // 3. Check topics
        self.check_topics(&mut health).await?;

        // 4. Check replication status
        self.check_replication(&mut health).await?;

        if health.healthy {
            info!("✓ Cluster health check passed");
        } else {
            info!("⚠ Cluster health check failed");
        }

        Ok(health)
    }

    /// Check broker connectivity
    async fn check_connectivity(&self, health: &mut ClusterHealth) -> Result<()> {
        debug!("Checking broker connectivity");

        let consumer: BaseConsumer = ClientConfig::new()
            .set("bootstrap.servers", &self.bootstrap_servers)
            .set("session.timeout.ms", "6000")
            .create()
            .context("Failed to create consumer")?;

        match consumer.fetch_metadata(None, Duration::from_secs(10)) {
            Ok(_) => {
                health.add_message("Kafka brokers are reachable".to_string());
                Ok(())
            }
            Err(e) => {
                health.mark_unhealthy(format!("Cannot connect to Kafka brokers: {}", e));
                Err(anyhow::anyhow!("Broker connectivity check failed: {}", e))
            }
        }
    }

    /// Check cluster metadata
    async fn check_metadata(&self, health: &mut ClusterHealth) -> Result<()> {
        debug!("Checking cluster metadata");

        let metadata = self
            .admin_client
            .inner()
            .fetch_metadata(None, Duration::from_secs(10))
            .context("Failed to fetch metadata")?;

        let broker_count = metadata.brokers().len();
        health.broker_count = broker_count;

        health.add_message(format!("Found {} brokers", broker_count));

        if broker_count < 3 {
            health.add_message(format!(
                "WARNING: Less than 3 brokers detected (recommended: 3+)"
            ));
        }

        Ok(())
    }

    /// Check topics
    async fn check_topics(&self, health: &mut ClusterHealth) -> Result<()> {
        debug!("Checking topics");

        let metadata = self
            .admin_client
            .inner()
            .fetch_metadata(None, Duration::from_secs(10))
            .context("Failed to fetch metadata")?;

        let topics: Vec<String> = metadata
            .topics()
            .iter()
            .map(|t| t.name().to_string())
            .collect();

        let topic_count = topics.len();
        let llm_topic_count = topics.iter().filter(|t| t.starts_with("llm-")).count();

        health.topic_count = topic_count;
        health.llm_topic_count = llm_topic_count;

        health.add_message(format!("Found {} total topics", topic_count));
        health.add_message(format!("Found {} LLM Analytics topics", llm_topic_count));

        if llm_topic_count < 10 {
            health.add_message(format!(
                "WARNING: Expected 14 LLM topics, found {}",
                llm_topic_count
            ));
        }

        Ok(())
    }

    /// Check replication status
    async fn check_replication(&self, health: &mut ClusterHealth) -> Result<()> {
        debug!("Checking replication status");

        let metadata = self
            .admin_client
            .inner()
            .fetch_metadata(None, Duration::from_secs(10))
            .context("Failed to fetch metadata")?;

        let mut under_replicated = 0;
        let mut offline = 0;

        for topic in metadata.topics() {
            for partition in topic.partitions() {
                let replicas = partition.replicas().len();
                let isr = partition.isr().len();

                // Under-replicated if ISR < replicas
                if isr < replicas {
                    under_replicated += 1;
                    debug!(
                        "Under-replicated partition: {} partition {} (ISR: {}/{})",
                        topic.name(),
                        partition.id(),
                        isr,
                        replicas
                    );
                }

                // Offline if no leader
                if partition.leader() < 0 {
                    offline += 1;
                    debug!("Offline partition: {} partition {}", topic.name(), partition.id());
                }
            }
        }

        health.under_replicated_partitions = under_replicated;
        health.offline_partitions = offline;

        if under_replicated == 0 {
            health.add_message("No under-replicated partitions".to_string());
        } else {
            health.add_message(format!(
                "WARNING: Found {} under-replicated partitions",
                under_replicated
            ));
        }

        if offline == 0 {
            health.add_message("No offline partitions".to_string());
        } else {
            health.mark_unhealthy(format!("Found {} offline partitions (CRITICAL)", offline));
        }

        Ok(())
    }
}
