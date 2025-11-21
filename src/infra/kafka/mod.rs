//! Kafka management infrastructure
//!
//! This module provides Kafka cluster management capabilities:
//! - Topic creation and configuration
//! - ACL management for security
//! - Cluster health verification
//! - Partition and replication status

pub mod topics;
pub mod acls;
pub mod verification;
pub mod types;

pub use topics::TopicManager;
pub use acls::AclManager;
pub use verification::ClusterVerifier;
pub use types::*;
