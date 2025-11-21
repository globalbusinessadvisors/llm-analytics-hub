//! Database connectivity validation
//!
//! Validates database connectivity and configuration

use anyhow::{Context, Result};
use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Api};
use tracing::debug;

use crate::infra::k8s::K8sClient;
use super::types::{CheckSeverity, ValidationCheck, ValidationResults};

/// Database validator
pub struct DatabaseValidator {
    client: K8sClient,
}

impl DatabaseValidator {
    /// Create new database validator
    pub fn new(client: K8sClient) -> Self {
        Self { client }
    }

    /// Run all database validation checks
    pub async fn validate(&self) -> Result<ValidationResults> {
        let mut results = ValidationResults::new("Database Connectivity");

        // Check PostgreSQL connectivity
        results.add_check(self.check_postgres_connectivity().await?);

        // Check database exists
        results.add_check(self.check_database_exists().await?);

        // Check TimescaleDB extension
        results.add_check(self.check_timescaledb_extension().await?);

        // Check Redis connectivity
        results.add_check(self.check_redis_connectivity().await?);

        // Check Kafka connectivity
        results.add_check(self.check_kafka_connectivity().await?);

        Ok(results)
    }

    /// Check PostgreSQL connectivity
    async fn check_postgres_connectivity(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("postgres-connectivity", "Database Connectivity", CheckSeverity::Critical);

        match self.get_timescaledb_pod().await? {
            Some(pod_name) => {
                // Try to connect to PostgreSQL
                let result = self
                    .client
                    .exec_in_pod(&pod_name, "pg_isready -U postgres")
                    .await;

                match result {
                    Ok(output) if output.contains("accepting connections") => {
                        Ok(check.pass("PostgreSQL is accepting connections"))
                    }
                    Ok(_) => Ok(check.fail("PostgreSQL is not ready")),
                    Err(e) => {
                        debug!("PostgreSQL connectivity check error: {}", e);
                        Ok(check.fail("Failed to check PostgreSQL connectivity"))
                    }
                }
            }
            None => Ok(check.skip("No TimescaleDB pod available")),
        }
    }

    /// Check database exists
    async fn check_database_exists(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("database-exists", "Database Connectivity", CheckSeverity::Important);

        match self.get_timescaledb_pod().await? {
            Some(pod_name) => {
                let result = self
                    .client
                    .exec_in_pod(&pod_name, "psql -U postgres -lqt")
                    .await;

                match result {
                    Ok(output) if output.contains("llm_analytics") => {
                        Ok(check.pass("Database 'llm_analytics' exists"))
                    }
                    Ok(_) => Ok(check.fail("Database 'llm_analytics' does not exist")),
                    Err(e) => {
                        debug!("Database check error: {}", e);
                        Ok(check.warn("Failed to verify database existence"))
                    }
                }
            }
            None => Ok(check.skip("No TimescaleDB pod available")),
        }
    }

    /// Check TimescaleDB extension
    async fn check_timescaledb_extension(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("timescaledb-extension", "Database Connectivity", CheckSeverity::Important);

        match self.get_timescaledb_pod().await? {
            Some(pod_name) => {
                let cmd = "psql -U postgres -d llm_analytics -c \"SELECT extname FROM pg_extension WHERE extname='timescaledb';\"";
                let result = self.client.exec_in_pod(&pod_name, cmd).await;

                match result {
                    Ok(output) if output.contains("timescaledb") => {
                        Ok(check.pass("TimescaleDB extension is installed"))
                    }
                    Ok(_) => Ok(check.warn("TimescaleDB extension may not be installed")),
                    Err(e) => {
                        debug!("TimescaleDB extension check error: {}", e);
                        Ok(check.warn("Failed to verify TimescaleDB extension"))
                    }
                }
            }
            None => Ok(check.skip("No TimescaleDB pod available")),
        }
    }

    /// Check Redis connectivity
    async fn check_redis_connectivity(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("redis-connectivity", "Database Connectivity", CheckSeverity::Important);

        match self.get_redis_pod().await? {
            Some(pod_name) => {
                let result = self.client.exec_in_pod(&pod_name, "redis-cli PING").await;

                match result {
                    Ok(output) if output.contains("PONG") => {
                        Ok(check.pass("Redis is responding"))
                    }
                    Ok(_) => Ok(check.fail("Redis not responding correctly")),
                    Err(e) => {
                        debug!("Redis connectivity check error: {}", e);
                        Ok(check.warn("Failed to check Redis connectivity"))
                    }
                }
            }
            None => Ok(check.skip("No Redis pod available")),
        }
    }

    /// Check Kafka connectivity
    async fn check_kafka_connectivity(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("kafka-connectivity", "Database Connectivity", CheckSeverity::Important);

        match self.get_kafka_pod().await? {
            Some(pod_name) => {
                // Try to list topics as a connectivity check
                let cmd = "kafka-topics.sh --bootstrap-server localhost:9092 --list";
                let result = self.client.exec_in_pod(&pod_name, cmd).await;

                match result {
                    Ok(_) => Ok(check.pass("Kafka broker is responding")),
                    Err(e) => {
                        debug!("Kafka connectivity check error: {}", e);
                        Ok(check.warn("Failed to check Kafka connectivity"))
                    }
                }
            }
            None => Ok(check.skip("No Kafka pod available")),
        }
    }

    /// Get TimescaleDB pod name
    async fn get_timescaledb_pod(&self) -> Result<Option<String>> {
        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let lp = ListParams::default().labels("app=timescaledb");
        let pods = pods_api.list(&lp).await.context("Failed to list TimescaleDB pods")?;

        Ok(pods
            .items
            .iter()
            .filter(|pod| {
                pod.status
                    .as_ref()
                    .and_then(|s| s.phase.as_deref())
                    .unwrap_or("") == "Running"
            })
            .next()
            .and_then(|pod| pod.metadata.name.clone()))
    }

    /// Get Redis pod name
    async fn get_redis_pod(&self) -> Result<Option<String>> {
        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let lp = ListParams::default().labels("app=redis");
        let pods = pods_api.list(&lp).await.context("Failed to list Redis pods")?;

        Ok(pods
            .items
            .iter()
            .filter(|pod| {
                pod.status
                    .as_ref()
                    .and_then(|s| s.phase.as_deref())
                    .unwrap_or("") == "Running"
            })
            .next()
            .and_then(|pod| pod.metadata.name.clone()))
    }

    /// Get Kafka pod name
    async fn get_kafka_pod(&self) -> Result<Option<String>> {
        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let lp = ListParams::default().labels("app=kafka");
        let pods = pods_api.list(&lp).await.context("Failed to list Kafka pods")?;

        Ok(pods
            .items
            .iter()
            .filter(|pod| {
                pod.status
                    .as_ref()
                    .and_then(|s| s.phase.as_deref())
                    .unwrap_or("") == "Running"
            })
            .next()
            .and_then(|pod| pod.metadata.name.clone()))
    }
}
