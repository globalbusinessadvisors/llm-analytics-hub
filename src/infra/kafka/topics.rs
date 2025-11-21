//! Kafka topic management

use anyhow::{Context, Result};
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::config::ClientConfig;
use std::time::Duration;
use tracing::{debug, info, warn};

use super::types::TopicConfig;

/// Topic manager for Kafka
pub struct TopicManager {
    admin_client: AdminClient<DefaultClientContext>,
    bootstrap_servers: String,
}

impl TopicManager {
    /// Create a new topic manager
    pub fn new(bootstrap_servers: impl Into<String>) -> Result<Self> {
        let bootstrap_servers = bootstrap_servers.into();

        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", &bootstrap_servers)
            .set("client.id", "llm-analytics-topic-manager")
            .create()
            .context("Failed to create Kafka admin client")?;

        Ok(Self {
            admin_client,
            bootstrap_servers,
        })
    }

    /// Create topics from configurations
    pub async fn create_topics(&self, configs: &[TopicConfig]) -> Result<Vec<String>> {
        info!("Creating {} topics on {}", configs.len(), self.bootstrap_servers);

        let mut created_topics = Vec::new();

        for config in configs {
            match self.create_topic(config).await {
                Ok(true) => {
                    info!("✓ Created topic: {} ({} partitions, RF={})",
                        config.name, config.partitions, config.replication_factor);
                    created_topics.push(config.name.clone());
                }
                Ok(false) => {
                    info!("✓ Topic already exists: {}", config.name);
                }
                Err(e) => {
                    warn!("⚠ Failed to create topic {}: {}", config.name, e);
                    return Err(e);
                }
            }
        }

        Ok(created_topics)
    }

    /// Create a single topic
    pub async fn create_topic(&self, config: &TopicConfig) -> Result<bool> {
        debug!("Creating topic: {}", config.name);
        debug!("  Partitions: {}", config.partitions);
        debug!("  Replication Factor: {}", config.replication_factor);
        debug!("  Config: {:?}", config.config);

        // Create the topic
        let new_topic = NewTopic::new(
            &config.name,
            config.partitions,
            TopicReplication::Fixed(config.replication_factor),
        );

        // Add configurations
        let new_topic = config.config.iter().fold(new_topic, |topic, (key, value)| {
            topic.set(key, value)
        });

        let opts = AdminOptions::new().operation_timeout(Some(Duration::from_secs(30)));

        let results = self
            .admin_client
            .create_topics(&[new_topic], &opts)
            .await
            .context("Failed to create topic")?;

        for result in results {
            match result {
                Ok(_) => return Ok(true),
                Err((topic, err)) => {
                    // Topic already exists is not an error
                    if err.to_string().contains("already exists") {
                        return Ok(false);
                    }
                    anyhow::bail!("Failed to create topic {}: {}", topic, err);
                }
            }
        }

        Ok(true)
    }

    /// List all topics
    pub async fn list_topics(&self) -> Result<Vec<String>> {
        debug!("Listing topics");

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

        debug!("Found {} topics", topics.len());
        Ok(topics)
    }

    /// List LLM Analytics topics
    pub async fn list_llm_topics(&self) -> Result<Vec<String>> {
        let all_topics = self.list_topics().await?;
        Ok(all_topics
            .into_iter()
            .filter(|t| t.starts_with("llm-"))
            .collect())
    }

    /// Delete topics
    pub async fn delete_topics(&self, topic_names: &[String]) -> Result<()> {
        info!("Deleting {} topics", topic_names.len());

        let opts = AdminOptions::new().operation_timeout(Some(Duration::from_secs(30)));

        let topic_refs: Vec<&str> = topic_names.iter().map(|s| s.as_str()).collect();

        let results = self
            .admin_client
            .delete_topics(&topic_refs, &opts)
            .await
            .context("Failed to delete topics")?;

        for result in results {
            match result {
                Ok(topic) => info!("✓ Deleted topic: {}", topic),
                Err((topic, err)) => {
                    warn!("⚠ Failed to delete topic {}: {}", topic, err);
                    anyhow::bail!("Failed to delete topic {}: {}", topic, err);
                }
            }
        }

        Ok(())
    }

    /// Describe a topic
    pub async fn describe_topic(&self, topic_name: &str) -> Result<TopicDescription> {
        debug!("Describing topic: {}", topic_name);

        let metadata = self
            .admin_client
            .inner()
            .fetch_metadata(Some(topic_name), Duration::from_secs(10))
            .context("Failed to fetch metadata")?;

        let topic = metadata
            .topics()
            .iter()
            .find(|t| t.name() == topic_name)
            .context("Topic not found")?;

        let partitions = topic.partitions().len();
        let replication_factor = topic
            .partitions()
            .first()
            .map(|p| p.replicas().len())
            .unwrap_or(0);

        Ok(TopicDescription {
            name: topic_name.to_string(),
            partitions: partitions as i32,
            replication_factor: replication_factor as i32,
        })
    }
}

/// Topic description
#[derive(Debug, Clone)]
pub struct TopicDescription {
    /// Topic name
    pub name: String,

    /// Number of partitions
    pub partitions: i32,

    /// Replication factor
    pub replication_factor: i32,
}
