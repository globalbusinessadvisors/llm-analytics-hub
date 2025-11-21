//! Terraform command executor

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;
use tracing::{debug, info};

use crate::common::progress::ProgressTracker;

/// Terraform command executor
pub struct TerraformExecutor {
    working_dir: PathBuf,
}

impl TerraformExecutor {
    /// Create a new Terraform executor
    pub fn new(working_dir: impl Into<PathBuf>) -> Self {
        Self {
            working_dir: working_dir.into(),
        }
    }

    /// Check if Terraform is installed
    pub async fn check_installed() -> Result<bool> {
        match Command::new("terraform")
            .arg("version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
        {
            Ok(status) => Ok(status.success()),
            Err(_) => Ok(false),
        }
    }

    /// Get Terraform version
    pub async fn version(&self) -> Result<String> {
        let output = Command::new("terraform")
            .arg("version")
            .current_dir(&self.working_dir)
            .output()
            .await
            .context("Failed to execute terraform version")?;

        if !output.status.success() {
            anyhow::bail!("Terraform version command failed");
        }

        String::from_utf8(output.stdout)
            .context("Failed to parse terraform version output")
    }

    /// Initialize Terraform
    pub async fn init(&self, show_progress: bool) -> Result<TerraformResult> {
        info!("Initializing Terraform in {:?}", self.working_dir);

        let spinner = if show_progress {
            Some(ProgressTracker::spinner("Initializing Terraform..."))
        } else {
            None
        };

        let output = Command::new("terraform")
            .arg("init")
            .arg("-upgrade")
            .current_dir(&self.working_dir)
            .output()
            .await
            .context("Failed to execute terraform init")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if let Some(s) = spinner {
            if output.status.success() {
                s.finish_success("Terraform initialized");
            } else {
                s.finish_error("Terraform init failed");
            }
        }

        if !output.status.success() {
            anyhow::bail!("Terraform init failed: {}", stderr);
        }

        Ok(TerraformResult {
            success: true,
            stdout,
            stderr,
        })
    }

    /// Validate Terraform configuration
    pub async fn validate(&self) -> Result<TerraformResult> {
        debug!("Validating Terraform configuration");

        let output = Command::new("terraform")
            .arg("validate")
            .current_dir(&self.working_dir)
            .output()
            .await
            .context("Failed to execute terraform validate")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            anyhow::bail!("Terraform validation failed: {}", stderr);
        }

        Ok(TerraformResult {
            success: true,
            stdout,
            stderr,
        })
    }

    /// Run Terraform plan
    pub async fn plan(&self, var_file: Option<&Path>) -> Result<TerraformResult> {
        info!("Running Terraform plan");

        let mut cmd = Command::new("terraform");
        cmd.arg("plan").current_dir(&self.working_dir);

        if let Some(vars) = var_file {
            cmd.arg("-var-file").arg(vars);
        }

        let output = cmd
            .output()
            .await
            .context("Failed to execute terraform plan")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            anyhow::bail!("Terraform plan failed: {}", stderr);
        }

        Ok(TerraformResult {
            success: true,
            stdout,
            stderr,
        })
    }

    /// Apply Terraform configuration
    pub async fn apply(
        &self,
        var_file: Option<&Path>,
        auto_approve: bool,
        show_progress: bool,
    ) -> Result<TerraformResult> {
        info!("Applying Terraform configuration");

        let spinner = if show_progress {
            Some(ProgressTracker::spinner("Applying Terraform..."))
        } else {
            None
        };

        let mut cmd = Command::new("terraform");
        cmd.arg("apply").current_dir(&self.working_dir);

        if let Some(vars) = var_file {
            cmd.arg("-var-file").arg(vars);
        }

        if auto_approve {
            cmd.arg("-auto-approve");
        }

        let output = cmd
            .output()
            .await
            .context("Failed to execute terraform apply")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if let Some(s) = spinner {
            if output.status.success() {
                s.finish_success("Terraform applied successfully");
            } else {
                s.finish_error("Terraform apply failed");
            }
        }

        if !output.status.success() {
            anyhow::bail!("Terraform apply failed: {}", stderr);
        }

        Ok(TerraformResult {
            success: true,
            stdout,
            stderr,
        })
    }

    /// Destroy Terraform-managed infrastructure
    pub async fn destroy(
        &self,
        var_file: Option<&Path>,
        auto_approve: bool,
        show_progress: bool,
    ) -> Result<TerraformResult> {
        info!("Destroying Terraform-managed infrastructure");

        let spinner = if show_progress {
            Some(ProgressTracker::spinner("Destroying infrastructure..."))
        } else {
            None
        };

        let mut cmd = Command::new("terraform");
        cmd.arg("destroy").current_dir(&self.working_dir);

        if let Some(vars) = var_file {
            cmd.arg("-var-file").arg(vars);
        }

        if auto_approve {
            cmd.arg("-auto-approve");
        }

        let output = cmd
            .output()
            .await
            .context("Failed to execute terraform destroy")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if let Some(s) = spinner {
            if output.status.success() {
                s.finish_success("Infrastructure destroyed");
            } else {
                s.finish_error("Terraform destroy failed");
            }
        }

        if !output.status.success() {
            anyhow::bail!("Terraform destroy failed: {}", stderr);
        }

        Ok(TerraformResult {
            success: true,
            stdout,
            stderr,
        })
    }

    /// Get Terraform output value
    pub async fn output(&self, name: &str) -> Result<String> {
        let output = Command::new("terraform")
            .arg("output")
            .arg("-raw")
            .arg(name)
            .current_dir(&self.working_dir)
            .output()
            .await
            .with_context(|| format!("Failed to get terraform output: {}", name))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to get output {}: {}", name, stderr);
        }

        Ok(String::from_utf8(output.stdout)
            .context("Failed to parse output")?
            .trim()
            .to_string())
    }
}

/// Terraform command type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerraformCommand {
    Init,
    Validate,
    Plan,
    Apply,
    Destroy,
}

/// Terraform execution result
#[derive(Debug, Clone)]
pub struct TerraformResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_terraform_check() {
        // Just test that the check doesn't panic
        let _ = TerraformExecutor::check_installed().await;
    }

    #[test]
    fn test_terraform_executor_creation() {
        let executor = TerraformExecutor::new("/tmp/terraform");
        assert_eq!(executor.working_dir, PathBuf::from("/tmp/terraform"));
    }
}
