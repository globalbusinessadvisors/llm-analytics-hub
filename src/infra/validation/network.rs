//! Network connectivity validation
//!
//! Validates network connectivity and DNS resolution

use anyhow::{Context, Result};
use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::api::networking::v1::Ingress;
use kube::{api::ListParams, Api};
use tracing::debug;

use crate::infra::k8s::K8sClient;
use super::types::{CheckSeverity, ValidationCheck, ValidationResults};

/// Network validator
pub struct NetworkValidator {
    client: K8sClient,
}

impl NetworkValidator {
    /// Create new network validator
    pub fn new(client: K8sClient) -> Self {
        Self { client }
    }

    /// Run all network validation checks
    pub async fn validate(&self) -> Result<ValidationResults> {
        let mut results = ValidationResults::new("Network Connectivity");

        // Check DNS resolution
        results.add_check(self.check_dns_resolution().await?);

        // Check service connectivity
        results.add_check(self.check_service_connectivity().await?);

        // Check ingress configuration
        results.add_check(self.check_ingress().await?);

        Ok(results)
    }

    /// Check DNS resolution
    async fn check_dns_resolution(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("dns-resolution", "Network Connectivity", CheckSeverity::Critical);

        // Create a temporary pod to test DNS
        let test_result = self
            .client
            .run_pod_command(
                "dns-test",
                "busybox:latest",
                &["nslookup", "kubernetes.default"],
            )
            .await;

        match test_result {
            Ok(_) => Ok(check.pass("DNS resolution working")),
            Err(e) => {
                debug!("DNS resolution check error: {}", e);
                Ok(check.fail("DNS resolution not working"))
            }
        }
    }

    /// Check service-to-service connectivity
    async fn check_service_connectivity(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("service-connectivity", "Network Connectivity", CheckSeverity::Important);

        // Get an API pod to test connectivity from
        match self.get_app_pod().await? {
            Some(pod_name) => {
                // Test TimescaleDB connectivity
                let db_check = self
                    .client
                    .exec_in_pod(&pod_name, "nc -zv timescaledb-service 5432")
                    .await;

                match db_check {
                    Ok(_) => Ok(check.pass("Service-to-service connectivity working")),
                    Err(e) => {
                        debug!("Service connectivity check error: {}", e);
                        Ok(check.warn("Some service connectivity issues detected"))
                    }
                }
            }
            None => Ok(check.skip("No application pod available for connectivity tests")),
        }
    }

    /// Check ingress configuration
    async fn check_ingress(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("ingress-configured", "Network Connectivity", CheckSeverity::Advisory);

        let ingress_api: Api<Ingress> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let ingresses = ingress_api
            .list(&ListParams::default())
            .await
            .context("Failed to list ingress resources")?;

        let ingress_count = ingresses.items.len();
        debug!("Ingress resources configured: {}", ingress_count);

        if ingress_count > 0 {
            Ok(check.pass(format!("Ingress resources configured ({} ingresses)", ingress_count)))
        } else {
            Ok(check.warn("No ingress resources configured - services not exposed externally"))
        }
    }

    /// Get application pod name
    async fn get_app_pod(&self) -> Result<Option<String>> {
        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let lp = ListParams::default().labels("app=analytics-api");
        let pods = pods_api.list(&lp).await.context("Failed to list application pods")?;

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
