//! Prerequisites validation
//!
//! Validates that required tools and access are available

use anyhow::Result;
use std::process::Command;
use tracing::debug;

use super::types::{CheckSeverity, ValidationCheck, ValidationResults};

/// Prerequisites validator
pub struct PrerequisiteValidator;

impl PrerequisiteValidator {
    /// Create new prerequisites validator
    pub fn new() -> Self {
        Self
    }

    /// Run all prerequisite checks
    pub async fn validate(&self) -> Result<ValidationResults> {
        let mut results = ValidationResults::new("Prerequisites");

        // Check kubectl
        results.add_check(self.check_kubectl().await);

        // Check helm
        results.add_check(self.check_helm().await);

        // Check cluster connectivity
        results.add_check(self.check_cluster_access().await);

        Ok(results)
    }

    /// Check kubectl is installed
    async fn check_kubectl(&self) -> ValidationCheck {
        let check = ValidationCheck::new("kubectl-installed", "Prerequisites", CheckSeverity::Critical);

        match Command::new("kubectl").arg("version").arg("--client").arg("--short").output() {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                let version = version.lines().next().unwrap_or("unknown");
                debug!("kubectl version: {}", version);
                check.pass(format!("kubectl installed ({})", version))
            }
            Ok(_) => check.fail("kubectl command failed"),
            Err(e) => {
                debug!("kubectl check error: {}", e);
                check.fail("kubectl not found - install kubectl to continue")
            }
        }
    }

    /// Check helm is installed
    async fn check_helm(&self) -> ValidationCheck {
        let check = ValidationCheck::new("helm-installed", "Prerequisites", CheckSeverity::Important);

        match Command::new("helm").arg("version").arg("--short").output() {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                let version = version.trim();
                debug!("helm version: {}", version);
                check.pass(format!("helm installed ({})", version))
            }
            Ok(_) => check.warn("helm command failed"),
            Err(e) => {
                debug!("helm check error: {}", e);
                check.warn("helm not found - some operations may not be available")
            }
        }
    }

    /// Check cluster access
    async fn check_cluster_access(&self) -> ValidationCheck {
        let check = ValidationCheck::new("cluster-access", "Prerequisites", CheckSeverity::Critical);

        match Command::new("kubectl").arg("cluster-info").output() {
            Ok(output) if output.status.success() => {
                check.pass("Kubernetes cluster is accessible")
            }
            Ok(_) => check.fail("Cannot connect to Kubernetes cluster - check kubeconfig"),
            Err(e) => {
                debug!("cluster access error: {}", e);
                check.fail("Failed to check cluster access")
            }
        }
    }
}

impl Default for PrerequisiteValidator {
    fn default() -> Self {
        Self::new()
    }
}
