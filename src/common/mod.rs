//! Common utilities for CLI operations
//!
//! This module provides shared functionality used across all CLI commands:
//! - Output formatting with colors and symbols
//! - Progress indicators for long-running operations
//! - Configuration management
//! - Command execution with retry logic

pub mod config;
pub mod executor;
pub mod output;
pub mod progress;

// Re-export commonly used types
pub use config::{CliConfig, ExecutionContext};
pub use executor::CommandExecutor;
pub use output::{print_error, print_info, print_success, print_warning, OutputFormat};
pub use progress::ProgressTracker;
