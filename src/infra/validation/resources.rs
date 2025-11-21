//! Resource utilization validation
//!
//! Validates resource usage and autoscaling configuration

use anyhow::{Context, Result};
use k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscaler;
use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Api};
use tracing::debug;

use crate::infra::k8s::K8sClient;
use super::types::{CheckSeverity, ValidationCheck, ValidationResults};

/// Resource validator
pub struct ResourceValidator {
    client: K8sClient,
}

impl ResourceValidator {
    /// Create new resource validator
    pub fn new(client: K8sClient) -> Self {
        Self { client }
    }

    /// Run all resource validation checks
    pub async fn validate(&self) -> Result<ValidationResults> {
        let mut results = ValidationResults::new("Resource Utilization");

        // Check node resource usage
        results.add_check(self.check_node_resources().await?);

        // Check pod resource requests
        results.add_check(self.check_pod_resources().await?);

        // Check HPA configuration
        results.add_check(self.check_hpa().await?);

        Ok(results)
    }

    /// Check node resource usage
    async fn check_node_resources(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("node-resources", "Resource Utilization", CheckSeverity::Important);

        // Note: kubectl top nodes requires metrics-server
        // We'll check if metrics-server is available
        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), "kube-system");
        let lp = ListParams::default().labels("k8s-app=metrics-server");
        let metrics_pods = pods_api
            .list(&lp)
            .await
            .context("Failed to list metrics-server pods")?;

        if metrics_pods.items.is_empty() {
            return Ok(check.warn("Metrics server not installed - cannot check node resource usage"));
        }

        // If metrics-server is available, we assume node resources are being monitored
        Ok(check.pass("Metrics server is available for resource monitoring"))
    }

    /// Check pod resource requests
    async fn check_pod_resources(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("pod-resource-requests", "Resource Utilization", CheckSeverity::Important);

        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let pods = pods_api
            .list(&ListParams::default())
            .await
            .context("Failed to list pods")?;

        let pods_without_requests: Vec<String> = pods
            .items
            .iter()
            .filter(|pod| {
                pod.spec
                    .as_ref()
                    .and_then(|spec| Some(&spec.containers))
                    .map(|containers| {
                        containers.iter().any(|c| {
                            c.resources
                                .as_ref()
                                .and_then(|r| r.requests.as_ref())
                                .is_none()
                        })
                    })
                    .unwrap_or(true)
            })
            .filter_map(|pod| pod.metadata.name.clone())
            .collect();

        debug!("Pods without resource requests: {}", pods_without_requests.len());

        if pods_without_requests.is_empty() {
            Ok(check.pass("All pods have resource requests defined"))
        } else {
            Ok(check.warn(format!("{} pod(s) without resource requests - scheduling may be inefficient", pods_without_requests.len())))
        }
    }

    /// Check HPA configuration
    async fn check_hpa(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("hpa-configured", "Resource Utilization", CheckSeverity::Advisory);

        let hpa_api: Api<HorizontalPodAutoscaler> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let hpas = hpa_api
            .list(&ListParams::default())
            .await
            .context("Failed to list HPAs")?;

        let hpa_count = hpas.items.len();
        debug!("HPAs configured: {}", hpa_count);

        if hpa_count > 0 {
            Ok(check.pass(format!("HPA configured ({} autoscalers)", hpa_count)))
        } else {
            Ok(check.warn("No HPA configured - automatic scaling not enabled"))
        }
    }
}
