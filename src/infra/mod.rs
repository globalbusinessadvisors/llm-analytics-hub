//! Infrastructure operations
//!
//! This module provides abstractions for infrastructure operations:
//! - Kubernetes client and operations
//! - Cloud provider abstractions (AWS, GCP, Azure)
//! - Terraform execution
//! - Validation infrastructure
//! - Kafka management
//! - Redis management
//! - Backup and restore operations

pub mod backup;
pub mod cloud;
pub mod k8s;
pub mod kafka;
pub mod redis;
pub mod terraform;
pub mod validation;
