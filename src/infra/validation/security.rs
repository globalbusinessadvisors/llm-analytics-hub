//! Security compliance validation
//!
//! Validates security configurations and compliance

use anyhow::{Context, Result};
use k8s_openapi::api::core::v1::{Pod, Secret};
use k8s_openapi::api::networking::v1::NetworkPolicy;
use k8s_openapi::api::policy::v1::PodDisruptionBudget;
use kube::{api::ListParams, Api};
use tracing::debug;

use crate::infra::k8s::K8sClient;
use super::types::{CheckSeverity, ValidationCheck, ValidationResults};

/// Security validator
pub struct SecurityValidator {
    client: K8sClient,
}

impl SecurityValidator {
    /// Create new security validator
    pub fn new(client: K8sClient) -> Self {
        Self { client }
    }

    /// Run all security validation checks
    pub async fn validate(&self) -> Result<ValidationResults> {
        let mut results = ValidationResults::new("Security Compliance");

        // Check for pods running as root
        results.add_check(self.check_root_pods().await?);

        // Check for privileged containers
        results.add_check(self.check_privileged_containers().await?);

        // Check network policies
        results.add_check(self.check_network_policies().await?);

        // Check pod disruption budgets
        results.add_check(self.check_pod_disruption_budgets().await?);

        // Check secrets configured
        results.add_check(self.check_secrets().await?);

        // Check resource limits
        results.add_check(self.check_resource_limits().await?);

        Ok(results)
    }

    /// Check for pods running as root
    async fn check_root_pods(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("no-root-pods", "Security Compliance", CheckSeverity::Important);

        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let pods = pods_api
            .list(&ListParams::default())
            .await
            .context("Failed to list pods")?;

        let root_pods: Vec<String> = pods
            .items
            .iter()
            .filter(|pod| {
                // Check if pod security context allows running as root
                pod.spec
                    .as_ref()
                    .and_then(|spec| spec.security_context.as_ref())
                    .and_then(|sc| sc.run_as_user)
                    .map(|uid| uid == 0)
                    .unwrap_or(true) // If not specified, may run as root
            })
            .filter_map(|pod| pod.metadata.name.clone())
            .collect();

        debug!("Pods potentially running as root: {}", root_pods.len());

        if root_pods.is_empty() {
            Ok(check.pass("No pods running as root"))
        } else {
            Ok(check.warn(format!("{} pod(s) may be running as root", root_pods.len())))
        }
    }

    /// Check for privileged containers
    async fn check_privileged_containers(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("no-privileged-containers", "Security Compliance", CheckSeverity::Critical);

        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let pods = pods_api
            .list(&ListParams::default())
            .await
            .context("Failed to list pods")?;

        let privileged_pods: Vec<String> = pods
            .items
            .iter()
            .filter(|pod| {
                pod.spec
                    .as_ref()
                    .and_then(|spec| Some(&spec.containers))
                    .map(|containers| {
                        containers.iter().any(|c| {
                            c.security_context
                                .as_ref()
                                .and_then(|sc| sc.privileged)
                                .unwrap_or(false)
                        })
                    })
                    .unwrap_or(false)
            })
            .filter_map(|pod| pod.metadata.name.clone())
            .collect();

        debug!("Privileged containers found: {}", privileged_pods.len());

        if privileged_pods.is_empty() {
            Ok(check.pass("No privileged containers"))
        } else {
            Ok(check.warn(format!("{} pod(s) with privileged containers", privileged_pods.len())))
        }
    }

    /// Check network policies
    async fn check_network_policies(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("network-policies", "Security Compliance", CheckSeverity::Important);

        let netpol_api: Api<NetworkPolicy> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let policies = netpol_api
            .list(&ListParams::default())
            .await
            .context("Failed to list network policies")?;

        let policy_count = policies.items.len();
        debug!("Network policies configured: {}", policy_count);

        if policy_count > 0 {
            Ok(check.pass(format!("Network policies configured ({} policies)", policy_count)))
        } else {
            Ok(check.warn("No network policies configured - network traffic is not restricted"))
        }
    }

    /// Check pod disruption budgets
    async fn check_pod_disruption_budgets(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("pod-disruption-budgets", "Security Compliance", CheckSeverity::Advisory);

        let pdb_api: Api<PodDisruptionBudget> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let pdbs = pdb_api
            .list(&ListParams::default())
            .await
            .context("Failed to list pod disruption budgets")?;

        let pdb_count = pdbs.items.len();
        debug!("Pod disruption budgets configured: {}", pdb_count);

        if pdb_count > 0 {
            Ok(check.pass(format!("Pod disruption budgets configured ({} PDBs)", pdb_count)))
        } else {
            Ok(check.warn("No pod disruption budgets configured - availability during updates may be affected"))
        }
    }

    /// Check secrets configured
    async fn check_secrets(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("secrets-configured", "Security Compliance", CheckSeverity::Important);

        let secrets_api: Api<Secret> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let secrets = secrets_api
            .list(&ListParams::default())
            .await
            .context("Failed to list secrets")?;

        // Filter out default service account tokens
        let user_secrets: Vec<_> = secrets
            .items
            .iter()
            .filter(|s| {
                s.type_.as_deref() != Some("kubernetes.io/service-account-token")
            })
            .collect();

        let secret_count = user_secrets.len();
        debug!("User secrets configured: {}", secret_count);

        if secret_count > 0 {
            Ok(check.pass(format!("Secrets configured ({} secrets)", secret_count)))
        } else {
            Ok(check.warn("No secrets configured - applications may not have credentials"))
        }
    }

    /// Check resource limits
    async fn check_resource_limits(&self) -> Result<ValidationCheck> {
        let check = ValidationCheck::new("resource-limits", "Security Compliance", CheckSeverity::Important);

        let pods_api: Api<Pod> = Api::namespaced(self.client.client().clone(), self.client.namespace());
        let pods = pods_api
            .list(&ListParams::default())
            .await
            .context("Failed to list pods")?;

        let pods_without_limits: Vec<String> = pods
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
                                .and_then(|r| r.limits.as_ref())
                                .is_none()
                        })
                    })
                    .unwrap_or(true)
            })
            .filter_map(|pod| pod.metadata.name.clone())
            .collect();

        debug!("Pods without resource limits: {}", pods_without_limits.len());

        if pods_without_limits.is_empty() {
            Ok(check.pass("All pods have resource limits defined"))
        } else {
            Ok(check.warn(format!("{} pod(s) without resource limits", pods_without_limits.len())))
        }
    }
}
