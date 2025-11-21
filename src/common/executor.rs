//! Command execution utilities with retry logic

use anyhow::{Context, Result};
use std::process::{Command, Output, Stdio};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};

/// Command executor with retry logic
pub struct CommandExecutor {
    /// Maximum retry attempts
    max_retries: u32,

    /// Delay between retries (milliseconds)
    retry_delay: u64,
}

impl CommandExecutor {
    /// Create a new command executor
    pub fn new() -> Self {
        Self {
            max_retries: 3,
            retry_delay: 1000,
        }
    }

    /// Create with custom retry settings
    pub fn with_retries(max_retries: u32, retry_delay_ms: u64) -> Self {
        Self {
            max_retries,
            retry_delay: retry_delay_ms,
        }
    }

    /// Execute a command with retry logic
    pub async fn execute(
        &self,
        command: &str,
        args: &[&str],
        working_dir: Option<&str>,
    ) -> Result<Output> {
        let mut last_error = None;

        for attempt in 1..=self.max_retries {
            debug!(
                "Executing command: {} {} (attempt {}/{})",
                command,
                args.join(" "),
                attempt,
                self.max_retries
            );

            let mut cmd = Command::new(command);
            cmd.args(args)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());

            if let Some(dir) = working_dir {
                cmd.current_dir(dir);
            }

            match cmd.output() {
                Ok(output) if output.status.success() => {
                    debug!("Command succeeded");
                    return Ok(output);
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    last_error = Some(anyhow::anyhow!(
                        "Command failed with exit code {:?}: {}",
                        output.status.code(),
                        stderr
                    ));

                    if attempt < self.max_retries {
                        warn!(
                            "Command failed (attempt {}/{}), retrying in {}ms",
                            attempt, self.max_retries, self.retry_delay
                        );
                        sleep(Duration::from_millis(self.retry_delay)).await;
                    }
                }
                Err(e) => {
                    last_error = Some(anyhow::anyhow!("Failed to execute command: {}", e));

                    if attempt < self.max_retries {
                        warn!(
                            "Command execution failed (attempt {}/{}), retrying in {}ms",
                            attempt, self.max_retries, self.retry_delay
                        );
                        sleep(Duration::from_millis(self.retry_delay)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Command failed after retries")))
    }

    /// Execute a command and return stdout as string
    pub async fn execute_stdout(
        &self,
        command: &str,
        args: &[&str],
        working_dir: Option<&str>,
    ) -> Result<String> {
        let output = self.execute(command, args, working_dir).await?;
        String::from_utf8(output.stdout).context("Failed to parse command output as UTF-8")
    }

    /// Check if a command exists in PATH
    pub fn command_exists(command: &str) -> bool {
        Command::new("which")
            .arg(command)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    /// Execute without retry for interactive commands
    pub async fn execute_once(
        command: &str,
        args: &[&str],
        working_dir: Option<&str>,
    ) -> Result<Output> {
        let mut cmd = Command::new(command);
        cmd.args(args);

        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }

        let output = cmd
            .output()
            .with_context(|| format!("Failed to execute: {}", command))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Command failed: {}", stderr);
        }

        Ok(output)
    }
}

impl Default for CommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_command_exists() {
        // Test with a command that should exist
        assert!(CommandExecutor::command_exists("echo"));
    }

    #[tokio::test]
    async fn test_execute_simple_command() {
        let executor = CommandExecutor::new();
        let result = executor.execute("echo", &["hello"], None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_stdout() {
        let executor = CommandExecutor::new();
        let result = executor.execute_stdout("echo", &["test"], None).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("test"));
    }
}
