//! Kafka topic management command

use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
use tracing::info;

use crate::common::{
    output::{print_header, print_success, CommandOutput, FormattedTable},
    ExecutionContext,
};
use crate::infra::kafka::{TopicManager, get_llm_topic_configs};

/// Kafka topic management arguments
#[derive(Debug, Args)]
pub struct KafkaTopicsArgs {
    /// Kafka bootstrap servers
    #[arg(long, env = "KAFKA_BOOTSTRAP_SERVERS", default_value = "kafka:9092")]
    pub bootstrap_servers: String,

    /// Action to perform
    #[command(subcommand)]
    pub action: TopicAction,
}

#[derive(Debug, clap::Subcommand)]
pub enum TopicAction {
    /// Create all LLM Analytics topics
    Create {
        /// Custom topic configuration file (YAML)
        #[arg(short, long)]
        config: Option<PathBuf>,
    },

    /// List all topics
    List {
        /// Filter to LLM topics only
        #[arg(short, long)]
        llm_only: bool,
    },

    /// Describe a topic
    Describe {
        /// Topic name
        topic: String,
    },

    /// Delete topics
    Delete {
        /// Topic names (comma-separated)
        topics: String,

        /// Force delete without confirmation
        #[arg(short, long)]
        force: bool,
    },
}

impl KafkaTopicsArgs {
    /// Execute topic management command
    pub async fn execute(&self, ctx: &ExecutionContext) -> Result<()> {
        match &self.action {
            TopicAction::Create { config } => self.create_topics(ctx, config.as_ref()).await,
            TopicAction::List { llm_only } => self.list_topics(ctx, *llm_only).await,
            TopicAction::Describe { topic } => self.describe_topic(ctx, topic).await,
            TopicAction::Delete { topics, force } => self.delete_topics(ctx, topics, *force).await,
        }
    }

    /// Create topics
    async fn create_topics(&self, ctx: &ExecutionContext, config_file: Option<&PathBuf>) -> Result<()> {
        print_header("Creating Kafka Topics");

        info!("Connecting to Kafka: {}", self.bootstrap_servers);

        if ctx.dry_run {
            println!("[DRY RUN] Would create topics on {}", self.bootstrap_servers);
            return Ok(());
        }

        // Get topic configurations
        let topic_configs = if let Some(_config_file) = config_file {
            // TODO: Load from YAML file
            anyhow::bail!("Custom config file not yet implemented");
        } else {
            get_llm_topic_configs()
        };

        println!("Creating {} LLM Analytics topics...\n", topic_configs.len());

        let manager = TopicManager::new(&self.bootstrap_servers)
            .context("Failed to create topic manager")?;

        let created = manager
            .create_topics(&topic_configs)
            .await
            .context("Failed to create topics")?;

        if ctx.json_output {
            let output = CommandOutput::success_with_data(
                format!("Created {} topics", created.len()),
                serde_json::json!({
                    "topics": created,
                    "total": created.len(),
                }),
            );
            output.output_json();
        } else {
            println!();
            print_success(&format!("Successfully created {} topics", created.len()));

            println!("\nCreated topics:");
            for topic in &created {
                println!("  • {}", topic);
            }
        }

        Ok(())
    }

    /// List topics
    async fn list_topics(&self, ctx: &ExecutionContext, llm_only: bool) -> Result<()> {
        print_header("Kafka Topics");

        let manager = TopicManager::new(&self.bootstrap_servers)
            .context("Failed to create topic manager")?;

        let topics = if llm_only {
            manager.list_llm_topics().await?
        } else {
            manager.list_topics().await?
        };

        if ctx.json_output {
            let output = CommandOutput::success_with_data(
                format!("Found {} topics", topics.len()),
                serde_json::json!({
                    "topics": topics,
                    "count": topics.len(),
                }),
            );
            output.output_json();
        } else {
            println!("\nFound {} topics:\n", topics.len());

            let mut table = FormattedTable::new(vec!["Topic Name"]);
            for topic in &topics {
                table.add_row(vec![topic.clone()]);
            }
            table.print();
        }

        Ok(())
    }

    /// Describe a topic
    async fn describe_topic(&self, ctx: &ExecutionContext, topic: &str) -> Result<()> {
        print_header(&format!("Topic: {}", topic));

        let manager = TopicManager::new(&self.bootstrap_servers)
            .context("Failed to create topic manager")?;

        let description = manager
            .describe_topic(topic)
            .await
            .context("Failed to describe topic")?;

        if ctx.json_output {
            let output = CommandOutput::success_with_data(
                "Topic description",
                serde_json::json!({
                    "name": description.name,
                    "partitions": description.partitions,
                    "replication_factor": description.replication_factor,
                }),
            );
            output.output_json();
        } else {
            println!("\nTopic: {}", description.name);
            println!("Partitions: {}", description.partitions);
            println!("Replication Factor: {}", description.replication_factor);
        }

        Ok(())
    }

    /// Delete topics
    async fn delete_topics(&self, ctx: &ExecutionContext, topics: &str, force: bool) -> Result<()> {
        let topic_list: Vec<String> = topics.split(',').map(|s| s.trim().to_string()).collect();

        print_header("Delete Kafka Topics");

        println!("Topics to delete:");
        for topic in &topic_list {
            println!("  • {}", topic);
        }
        println!();

        if !force && !ctx.dry_run {
            println!("WARNING: This will permanently delete these topics and all their data!");
            println!("Use --force to confirm deletion");
            anyhow::bail!("Deletion not confirmed");
        }

        if ctx.dry_run {
            println!("[DRY RUN] Would delete {} topics", topic_list.len());
            return Ok(());
        }

        let manager = TopicManager::new(&self.bootstrap_servers)
            .context("Failed to create topic manager")?;

        manager
            .delete_topics(&topic_list)
            .await
            .context("Failed to delete topics")?;

        print_success(&format!("Successfully deleted {} topics", topic_list.len()));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_parsing() {
        let topics = "topic1, topic2, topic3";
        let list: Vec<String> = topics.split(',').map(|s| s.trim().to_string()).collect();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0], "topic1");
    }
}
