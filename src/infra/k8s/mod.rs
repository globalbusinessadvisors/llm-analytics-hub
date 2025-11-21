//! Kubernetes operations module
//!
//! Provides type-safe Kubernetes operations using kube-rs.

pub mod client;
pub mod deployment;
pub mod health;
pub mod resources;

pub use client::K8sClient;
pub use deployment::{DeploymentManager, DeploymentOptions};
pub use health::HealthChecker;
pub use resources::ResourceManager;
