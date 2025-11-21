//! Service availability validation
//!
//! Validates service deployment status and availability

use anyhow::{Context, Result};
use k8s_openapi::api::core::v1::{Pod, Service};
use kube::{api::ListParams, Api};
use tracing::debug;

use crate::infra::k8s::K8sClient;
use super::types::{CheckSeverity, ValidationCheck, ValidationResults};

/// Service validator
pub struct ServiceValidator {
    client: K8sClient,
}

impl ServiceValidator {
    /// Create new service validator
    pub fn new(client: K8sClient) -> Self {
        Self { client }
    }

    /// Run all service validation checks
    pub async fn validate(&self) -> Result<ValidationResults> {
        let mut results = ValidationResults::new("Service Availability");

        // Check application pods
        results.add_check(self.check_app_pods().await?);

        // Check pod readiness
        results.add_check(self.check_pod_readiness().await?);

        // Check TimescaleDB
        results.add_check(self.check_timescaledb().await?);

        // Check Redis
        results.add_check(self.check_redis().await?);

        // Check Kafka
        results.add_check(self.check_kafka().await?);

        // Check services configured
        results.add_check(self.check_services().await?);

        Ok(results)
    }

    /// Check application pods
    async fn check_app_pods(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("app-pods-running", "Service Availability", CheckSeverity::Critical);

        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let lp = ListParams::default().labels("app=analytics-api");
        let pods = pods_api
            .list(&lp)
            .await
            .context("Failed to list application pods")?;

        let total_pods = pods.items.len();
        if total_pods == 0 {
            return Ok(check.warn("No application pods found - may not be deployed yet"));
        }

        let running_pods = pods
            .items
            .iter()
            .filter(|pod| {
                pod.status
                    .as_ref()
                    .and_then(|s| s.phase.as_deref())
                    .unwrap_or("") == "Running"
            })
            .count();

        debug!("Application pods: {}/{} running", running_pods, total_pods);

        if running_pods == total_pods {
            Ok(check.pass(format!("All application pods running ({}/{})", running_pods, total_pods)))
        } else if running_pods > 0 {
            Ok(check.warn(format!("Some application pods not running ({}/{})", running_pods, total_pods)))
        } else {
            Ok(check.fail("No application pods running"))
        }
    }

    /// Check pod readiness
    async fn check_pod_readiness(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("pods-ready", "Service Availability", CheckSeverity::Important);

        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let lp = ListParams::default().labels("app=analytics-api");
        let pods = pods_api
            .list(&lp)
            .await
            .context("Failed to list pods")?;

        if pods.items.is_empty() {
            return Ok(check.skip("No pods to check readiness"));
        }

        let ready_pods = pods
            .items
            .iter()
            .filter(|pod| {
                pod.status
                    .as_ref()
                    .and_then(|s| s.conditions.as_ref())
                    .map(|conditions| {
                        conditions.iter().any(|c| {
                            c.type_ == "Ready" && c.status == "True"
                        })
                    })
                    .unwrap_or(false)
            })
            .count();

        let total_pods = pods.items.len();
        debug!("Ready pods: {}/{}", ready_pods, total_pods);

        if ready_pods == total_pods {
            Ok(check.pass(format!("All pods ready ({}/{})", ready_pods, total_pods)))
        } else {
            Ok(check.warn(format!("Some pods not ready ({}/{})", ready_pods, total_pods)))
        }
    }

    /// Check TimescaleDB
    async fn check_timescaledb(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("timescaledb-running", "Service Availability", CheckSeverity::Critical);

        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let lp = ListParams::default().labels("app=timescaledb");
        let pods = pods_api
            .list(&lp)
            .await
            .context("Failed to list TimescaleDB pods")?;

        let running_pods = pods
            .items
            .iter()
            .filter(|pod| {
                pod.status
                    .as_ref()
                    .and_then(|s| s.phase.as_deref())
                    .unwrap_or("") == "Running"
            })
            .count();

        if running_pods > 0 {
            Ok(check.pass(format!("TimescaleDB is running ({} pod(s))", running_pods)))
        } else {
            Ok(check.fail("TimescaleDB is not running"))
        }
    }

    /// Check Redis
    async fn check_redis(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("redis-cluster", "Service Availability", CheckSeverity::Important);

        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let lp = ListParams::default().labels("app=redis");
        let pods = pods_api
            .list(&lp)
            .await
            .context("Failed to list Redis pods")?;

        let running_pods = pods
            .items
            .iter()
            .filter(|pod| {
                pod.status
                    .as_ref()
                    .and_then(|s| s.phase.as_deref())
                    .unwrap_or("") == "Running"
            })
            .count();

        if running_pods >= 3 {
            Ok(check.pass(format!("Redis cluster is running ({} pods)", running_pods)))
        } else if running_pods > 0 {
            Ok(check.warn(format!("Redis cluster partially running ({} pods, expected 3+)", running_pods)))
        } else {
            Ok(check.fail("Redis cluster is not running"))
        }
    }

    /// Check Kafka
    async fn check_kafka(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("kafka-cluster", "Service Availability", CheckSeverity::Important);

        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let lp = ListParams::default().labels("app=kafka");
        let pods = pods_api
            .list(&lp)
            .await
            .context("Failed to list Kafka pods")?;

        let running_pods = pods
            .items
            .iter()
            .filter(|pod| {
                pod.status
                    .as_ref()
                    .and_then(|s| s.phase.as_deref())
                    .unwrap_or("") == "Running"
            })
            .count();

        if running_pods >= 3 {
            Ok(check.pass(format!("Kafka cluster is running ({} pods)", running_pods)))
        } else if running_pods > 0 {
            Ok(check.warn(format!("Kafka cluster partially running ({} pods, expected 3+)", running_pods)))
        } else {
            Ok(check.fail("Kafka cluster is not running"))
        }
    }

    /// Check services configured
    async fn check_services(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("services-configured", "Service Availability", CheckSeverity::Important);

        let services_api: Api<Service> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let services = services_api
            .list(&ListParams::default())
            .await
            .context("Failed to list services")?;

        let service_count = services.items.len();
        debug!("Services configured: {}", service_count);

        if service_count > 0 {
            Ok(check.pass(format!("Services configured ({} services)", service_count)))
        } else {
            Ok(check.fail("No services configured"))
        }
    }
}
