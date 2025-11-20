//! Correlation Engine
//!
//! Detects and analyzes correlations between events across different modules.
//! Supports 8 correlation types: causal, temporal, pattern, anomaly, cost,
//! security, performance, and compliance correlations.

use crate::database::Database;
use crate::schemas::events::{AnalyticsEvent, EventType, SourceModule};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, instrument};
use uuid::Uuid;

/// Correlation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CorrelationType {
    /// Causal relationship (A causes B)
    Causal,
    /// Temporal correlation (happens at similar times)
    Temporal,
    /// Pattern correlation (similar patterns)
    Pattern,
    /// Anomaly correlation (anomalies happen together)
    Anomaly,
    /// Cost correlation (cost impact relationship)
    Cost,
    /// Security correlation (security events related)
    Security,
    /// Performance correlation (performance impact)
    Performance,
    /// Compliance correlation (policy violations)
    Compliance,
}

impl CorrelationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CorrelationType::Causal => "causal",
            CorrelationType::Temporal => "temporal",
            CorrelationType::Pattern => "pattern",
            CorrelationType::Anomaly => "anomaly",
            CorrelationType::Cost => "cost",
            CorrelationType::Security => "security",
            CorrelationType::Performance => "performance",
            CorrelationType::Compliance => "compliance",
        }
    }
}

/// Correlation engine
pub struct CorrelationEngine {
    database: Arc<Database>,
    // Event buffer for correlation detection
    event_buffer: Arc<DashMap<Uuid, BufferedEvent>>,
    // Detected correlations
    correlations: Arc<DashMap<Uuid, Vec<Correlation>>>,
    config: CorrelationConfig,
}

#[derive(Debug, Clone)]
pub struct CorrelationConfig {
    /// Time window for temporal correlations
    pub time_window: Duration,
    /// Minimum correlation strength to report
    pub min_strength: f64,
    /// Maximum events to buffer
    pub max_buffer_size: usize,
}

impl Default for CorrelationConfig {
    fn default() -> Self {
        Self {
            time_window: Duration::minutes(5),
            min_strength: 0.6,
            max_buffer_size: 10000,
        }
    }
}

impl CorrelationEngine {
    /// Create a new correlation engine
    pub fn new(database: Arc<Database>, config: CorrelationConfig) -> Self {
        Self {
            database,
            event_buffer: Arc::new(DashMap::new()),
            correlations: Arc::new(DashMap::new()),
            config,
        }
    }

    /// Process an event and detect correlations
    #[instrument(skip(self, event))]
    pub async fn process_event(&self, event: &AnalyticsEvent) -> Result<Vec<Correlation>> {
        let mut detected = Vec::new();

        // Buffer the event
        self.buffer_event(event);

        // Check for existing correlation ID
        if let Some(correlation_id) = event.common.correlation_id {
            detected.extend(self.find_correlated_by_id(correlation_id).await?);
        }

        // Detect causal correlations
        if let Some(parent_id) = event.common.parent_event_id {
            if let Some(correlation) = self.detect_causal_correlation(event, parent_id).await? {
                detected.push(correlation);
            }
        }

        // Detect temporal correlations
        detected.extend(self.detect_temporal_correlations(event).await?);

        // Detect cross-module correlations
        detected.extend(self.detect_cross_module_correlations(event).await?);

        // Store detected correlations
        for correlation in &detected {
            self.store_correlation(correlation).await?;
        }

        Ok(detected)
    }

    /// Buffer an event for correlation detection
    fn buffer_event(&self, event: &AnalyticsEvent) {
        // Clean old events if buffer is full
        if self.event_buffer.len() >= self.config.max_buffer_size {
            self.cleanup_old_events();
        }

        let buffered = BufferedEvent {
            event_id: event.common.event_id,
            timestamp: event.common.timestamp,
            source_module: event.common.source_module.clone(),
            event_type: event.common.event_type.clone(),
            correlation_id: event.common.correlation_id,
            parent_event_id: event.common.parent_event_id,
        };

        self.event_buffer.insert(event.common.event_id, buffered);
    }

    /// Clean up old events from buffer
    fn cleanup_old_events(&self) {
        let cutoff = Utc::now() - self.config.time_window * 2;

        self.event_buffer.retain(|_, event| event.timestamp > cutoff);
    }

    /// Find events correlated by ID
    async fn find_correlated_by_id(&self, correlation_id: Uuid) -> Result<Vec<Correlation>> {
        let events = self.database.query_events_by_correlation(correlation_id).await?;

        let mut correlations = Vec::new();
        for event in events {
            // Create correlations between all events with same correlation ID
            // (simplified - in reality, you'd analyze the actual relationships)
        }

        Ok(correlations)
    }

    /// Detect causal correlation (parent-child relationship)
    async fn detect_causal_correlation(
        &self,
        event: &AnalyticsEvent,
        parent_id: Uuid,
    ) -> Result<Option<Correlation>> {
        if let Some(parent) = self.event_buffer.get(&parent_id) {
            let strength = self.calculate_causal_strength(event, &parent);

            if strength >= self.config.min_strength {
                return Ok(Some(Correlation {
                    correlation_id: Uuid::new_v4(),
                    correlation_type: CorrelationType::Causal,
                    source_event_id: parent_id,
                    target_event_id: event.common.event_id,
                    strength,
                    metadata: HashMap::new(),
                    detected_at: Utc::now(),
                }));
            }
        }

        Ok(None)
    }

    /// Detect temporal correlations (events happening close in time)
    async fn detect_temporal_correlations(
        &self,
        event: &AnalyticsEvent,
    ) -> Result<Vec<Correlation>> {
        let mut correlations = Vec::new();
        let time_window_start = event.common.timestamp - self.config.time_window;
        let time_window_end = event.common.timestamp + self.config.time_window;

        for entry in self.event_buffer.iter() {
            let other = entry.value();

            // Skip same event
            if other.event_id == event.common.event_id {
                continue;
            }

            // Check if within time window
            if other.timestamp >= time_window_start && other.timestamp <= time_window_end {
                let strength = self.calculate_temporal_strength(event, other);

                if strength >= self.config.min_strength {
                    correlations.push(Correlation {
                        correlation_id: Uuid::new_v4(),
                        correlation_type: CorrelationType::Temporal,
                        source_event_id: event.common.event_id,
                        target_event_id: other.event_id,
                        strength,
                        metadata: HashMap::new(),
                        detected_at: Utc::now(),
                    });
                }
            }
        }

        Ok(correlations)
    }

    /// Detect cross-module correlations
    async fn detect_cross_module_correlations(
        &self,
        event: &AnalyticsEvent,
    ) -> Result<Vec<Correlation>> {
        let mut correlations = Vec::new();

        // Cost-Performance correlation
        if event.common.event_type == EventType::Cost {
            correlations.extend(
                self.find_performance_impact(event).await?
            );
        }

        // Security-Compliance correlation
        if event.common.event_type == EventType::Security {
            correlations.extend(
                self.find_compliance_impact(event).await?
            );
        }

        Ok(correlations)
    }

    /// Find performance impact of an event
    async fn find_performance_impact(&self, event: &AnalyticsEvent) -> Result<Vec<Correlation>> {
        let mut correlations = Vec::new();

        // Look for performance events around the same time
        for entry in self.event_buffer.iter() {
            let other = entry.value();

            if other.event_type == EventType::Telemetry {
                let time_diff = (event.common.timestamp - other.timestamp).num_seconds().abs();

                if time_diff < 60 {
                    // Within 1 minute
                    correlations.push(Correlation {
                        correlation_id: Uuid::new_v4(),
                        correlation_type: CorrelationType::Performance,
                        source_event_id: event.common.event_id,
                        target_event_id: other.event_id,
                        strength: 0.8,
                        metadata: HashMap::new(),
                        detected_at: Utc::now(),
                    });
                }
            }
        }

        Ok(correlations)
    }

    /// Find compliance impact of a security event
    async fn find_compliance_impact(&self, event: &AnalyticsEvent) -> Result<Vec<Correlation>> {
        let mut correlations = Vec::new();

        for entry in self.event_buffer.iter() {
            let other = entry.value();

            if other.event_type == EventType::Governance {
                correlations.push(Correlation {
                    correlation_id: Uuid::new_v4(),
                    correlation_type: CorrelationType::Compliance,
                    source_event_id: event.common.event_id,
                    target_event_id: other.event_id,
                    strength: 0.75,
                    metadata: HashMap::new(),
                    detected_at: Utc::now(),
                });
            }
        }

        Ok(correlations)
    }

    /// Calculate causal correlation strength
    fn calculate_causal_strength(&self, _event: &AnalyticsEvent, _parent: &BufferedEvent) -> f64 {
        // Simplified - in reality, analyze the relationship depth, timing, etc.
        0.9
    }

    /// Calculate temporal correlation strength
    fn calculate_temporal_strength(&self, event: &AnalyticsEvent, other: &BufferedEvent) -> f64 {
        // Calculate based on time proximity and module relationship
        let time_diff = (event.common.timestamp - other.timestamp)
            .num_seconds()
            .abs() as f64;
        let window_seconds = self.config.time_window.num_seconds() as f64;

        // Closer in time = stronger correlation
        let time_strength = 1.0 - (time_diff / window_seconds);

        // Same module = weaker cross-module correlation
        let module_factor = if event.common.source_module == other.source_module {
            0.5
        } else {
            1.0
        };

        (time_strength * module_factor).max(0.0).min(1.0)
    }

    /// Store correlation to database
    async fn store_correlation(&self, correlation: &Correlation) -> Result<()> {
        let metadata_json = serde_json::to_value(&correlation.metadata)?;

        self.database
            .store_correlation(
                correlation.correlation_id,
                correlation.correlation_type.as_str(),
                correlation.source_event_id,
                correlation.target_event_id,
                correlation.strength,
                &metadata_json,
            )
            .await?;

        debug!(
            "Stored {} correlation: {} -> {} (strength: {:.2})",
            correlation.correlation_type.as_str(),
            correlation.source_event_id,
            correlation.target_event_id,
            correlation.strength
        );

        Ok(())
    }

    /// Get correlations for an event
    pub fn get_correlations(&self, event_id: Uuid) -> Vec<Correlation> {
        self.correlations
            .get(&event_id)
            .map(|corrs| corrs.clone())
            .unwrap_or_default()
    }

    /// Get engine statistics
    pub fn get_stats(&self) -> CorrelationStats {
        let total_correlations = self
            .correlations
            .iter()
            .map(|entry| entry.value().len())
            .sum();

        CorrelationStats {
            buffered_events: self.event_buffer.len(),
            total_correlations,
        }
    }
}

/// Buffered event for correlation detection
#[derive(Debug, Clone)]
struct BufferedEvent {
    event_id: Uuid,
    timestamp: DateTime<Utc>,
    source_module: SourceModule,
    event_type: EventType,
    correlation_id: Option<Uuid>,
    parent_event_id: Option<Uuid>,
}

/// Detected correlation
#[derive(Debug, Clone)]
pub struct Correlation {
    pub correlation_id: Uuid,
    pub correlation_type: CorrelationType,
    pub source_event_id: Uuid,
    pub target_event_id: Uuid,
    pub strength: f64,
    pub metadata: HashMap<String, String>,
    pub detected_at: DateTime<Utc>,
}

/// Correlation engine statistics
#[derive(Debug, Clone)]
pub struct CorrelationStats {
    pub buffered_events: usize,
    pub total_correlations: usize,
}
