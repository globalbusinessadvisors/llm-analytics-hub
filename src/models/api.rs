//! API Response Models
//!
//! Query result formats, pagination structures, error response schemas,
//! and streaming data formats for the analytics hub API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// STANDARD API RESPONSE WRAPPER
// ============================================================================

/// Standard API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Response status
    pub status: ResponseStatus,

    /// Response data (present on success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// Error details (present on error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,

    /// Response metadata
    pub meta: ResponseMetadata,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: ResponseStatus::Success,
            data: Some(data),
            error: None,
            meta: ResponseMetadata::default(),
        }
    }

    pub fn error(error: ApiError) -> Self {
        Self {
            status: ResponseStatus::Error,
            data: None,
            error: Some(error),
            meta: ResponseMetadata::default(),
        }
    }

    pub fn with_meta(mut self, meta: ResponseMetadata) -> Self {
        self.meta = meta;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Success,
    Error,
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Request identifier for tracing
    pub request_id: Uuid,

    /// Server timestamp
    pub timestamp: DateTime<Utc>,

    /// API version
    pub api_version: String,

    /// Response time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_ms: Option<u64>,

    /// Additional metadata
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Default for ResponseMetadata {
    fn default() -> Self {
        Self {
            request_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            api_version: "1.0.0".to_string(),
            response_time_ms: None,
            extra: HashMap::new(),
        }
    }
}

// ============================================================================
// PAGINATION
// ============================================================================

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// Response status
    pub status: ResponseStatus,

    /// Page data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<T>>,

    /// Pagination metadata
    pub pagination: PaginationMetadata,

    /// Error details (present on error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,

    /// Response metadata
    pub meta: ResponseMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMetadata {
    /// Current page number (1-indexed)
    pub page: u32,

    /// Items per page
    pub per_page: u32,

    /// Total number of items
    pub total_items: u64,

    /// Total number of pages
    pub total_pages: u32,

    /// Whether there is a next page
    pub has_next: bool,

    /// Whether there is a previous page
    pub has_previous: bool,

    /// Links to related pages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<PaginationLinks>,
}

impl PaginationMetadata {
    pub fn new(page: u32, per_page: u32, total_items: u64) -> Self {
        let total_pages = ((total_items as f64) / (per_page as f64)).ceil() as u32;
        Self {
            page,
            per_page,
            total_items,
            total_pages,
            has_next: page < total_pages,
            has_previous: page > 1,
            links: None,
        }
    }

    pub fn with_links(mut self, base_url: &str) -> Self {
        self.links = Some(PaginationLinks::new(base_url, self.page, self.total_pages));
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationLinks {
    /// Link to first page
    pub first: String,

    /// Link to last page
    pub last: String,

    /// Link to next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,

    /// Link to previous page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,

    /// Link to current page
    pub self_link: String,
}

impl PaginationLinks {
    pub fn new(base_url: &str, current_page: u32, total_pages: u32) -> Self {
        Self {
            first: format!("{}?page=1", base_url),
            last: format!("{}?page={}", base_url, total_pages),
            next: if current_page < total_pages {
                Some(format!("{}?page={}", base_url, current_page + 1))
            } else {
                None
            },
            prev: if current_page > 1 {
                Some(format!("{}?page={}", base_url, current_page - 1))
            } else {
                None
            },
            self_link: format!("{}?page={}", base_url, current_page),
        }
    }
}

/// Pagination parameters for requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: u32,

    /// Items per page
    #[serde(default = "default_per_page")]
    pub per_page: u32,

    /// Sort field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    /// Sort order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    50
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 50,
            sort_by: None,
            sort_order: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

// ============================================================================
// ERROR RESPONSES
// ============================================================================

/// API error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error code (machine-readable)
    pub code: String,

    /// Error message (human-readable)
    pub message: String,

    /// HTTP status code
    pub status_code: u16,

    /// Detailed error information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ErrorDetails>,

    /// Field-specific errors (for validation errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<HashMap<String, Vec<String>>>,

    /// Timestamp when error occurred
    pub timestamp: DateTime<Utc>,
}

impl ApiError {
    pub fn new(code: impl Into<String>, message: impl Into<String>, status_code: u16) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            status_code,
            details: None,
            field_errors: None,
            timestamp: Utc::now(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new("bad_request", message, 400)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new("unauthorized", message, 401)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new("forbidden", message, 403)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new("not_found", message, 404)
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new("internal_error", message, 500)
    }

    pub fn with_details(mut self, details: ErrorDetails) -> Self {
        self.details = Some(details);
        self
    }

    pub fn with_field_errors(mut self, errors: HashMap<String, Vec<String>>) -> Self {
        self.field_errors = Some(errors);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    /// Error trace/stack trace (for debugging)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<String>,

    /// Additional context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<HashMap<String, serde_json::Value>>,

    /// Suggested fixes or actions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<String>>,

    /// Documentation link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
}

// ============================================================================
// QUERY RESULT FORMATS
// ============================================================================

/// Generic query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult<T> {
    /// Query identifier
    pub query_id: Uuid,

    /// Query execution status
    pub status: QueryStatus,

    /// Result data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// Query execution metrics
    pub metrics: QueryMetrics,

    /// Warnings encountered during query execution
    #[serde(default)]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryStatus {
    Success,
    PartialSuccess,
    Failed,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMetrics {
    /// Query execution time in milliseconds
    pub execution_time_ms: u64,

    /// Number of records scanned
    pub records_scanned: u64,

    /// Number of records returned
    pub records_returned: u64,

    /// Data processed in bytes
    pub bytes_processed: u64,

    /// Whether results were cached
    pub from_cache: bool,

    /// Cache TTL if cached (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ttl: Option<u32>,
}

/// Time-series query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesQueryResult {
    /// Query specification
    pub query: String,

    /// Time range
    pub time_range: TimeRange,

    /// Result series
    pub series: Vec<SeriesData>,

    /// Query metrics
    pub metrics: QueryMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesData {
    /// Series name
    pub name: String,

    /// Series tags
    #[serde(default)]
    pub tags: HashMap<String, String>,

    /// Data points
    pub points: Vec<DataPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Value
    pub value: f64,

    /// Additional fields
    #[serde(flatten)]
    pub fields: HashMap<String, serde_json::Value>,
}

/// Aggregated metrics query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsQueryResult {
    /// Metric name
    pub metric: String,

    /// Aggregation window
    pub window: String,

    /// Aggregated values
    pub values: Vec<AggregatedValue>,

    /// Query metrics
    pub metrics: QueryMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedValue {
    pub timestamp: DateTime<Utc>,
    pub avg: f64,
    pub min: f64,
    pub max: f64,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
    pub count: u64,
}

// ============================================================================
// STREAMING RESPONSE FORMATS
// ============================================================================

/// Streaming event wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent<T> {
    /// Event identifier
    pub event_id: Uuid,

    /// Event type
    pub event_type: StreamEventType,

    /// Event data
    pub data: T,

    /// Sequence number (for ordering)
    pub sequence: u64,

    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StreamEventType {
    Data,
    Heartbeat,
    Error,
    Complete,
}

/// Server-Sent Events (SSE) format
#[derive(Debug, Clone)]
pub struct SseMessage {
    /// Event type
    pub event: Option<String>,

    /// Data payload
    pub data: String,

    /// Event ID (for reconnection)
    pub id: Option<String>,

    /// Retry interval in milliseconds
    pub retry: Option<u32>,
}

impl SseMessage {
    pub fn data(data: String) -> Self {
        Self {
            event: None,
            data,
            id: None,
            retry: None,
        }
    }

    pub fn event(event: String, data: String) -> Self {
        Self {
            event: Some(event),
            data,
            id: None,
            retry: None,
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        if let Some(event) = &self.event {
            result.push_str(&format!("event: {}\n", event));
        }

        if let Some(id) = &self.id {
            result.push_str(&format!("id: {}\n", id));
        }

        if let Some(retry) = &self.retry {
            result.push_str(&format!("retry: {}\n", retry));
        }

        result.push_str(&format!("data: {}\n\n", self.data));
        result
    }
}

/// Batch response for bulk operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResponse<T> {
    /// Batch identifier
    pub batch_id: Uuid,

    /// Total items in batch
    pub total_items: usize,

    /// Successfully processed items
    pub success_count: usize,

    /// Failed items
    pub failure_count: usize,

    /// Results for each item
    pub results: Vec<BatchItemResult<T>>,

    /// Overall batch status
    pub status: BatchStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchItemResult<T> {
    /// Item index in the batch
    pub index: usize,

    /// Item identifier
    pub item_id: Option<String>,

    /// Processing status
    pub status: ItemStatus,

    /// Result data (on success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// Error details (on failure)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ItemStatus {
    Success,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BatchStatus {
    AllSuccess,
    PartialSuccess,
    AllFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_success() {
        let response = ApiResponse::success("test data");
        assert_eq!(response.status, ResponseStatus::Success);
        assert!(response.data.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_api_response_error() {
        let error = ApiError::not_found("Resource not found");
        let response: ApiResponse<String> = ApiResponse::error(error);
        assert_eq!(response.status, ResponseStatus::Error);
        assert!(response.data.is_none());
        assert!(response.error.is_some());
    }

    #[test]
    fn test_pagination_metadata() {
        let pagination = PaginationMetadata::new(2, 50, 250);
        assert_eq!(pagination.page, 2);
        assert_eq!(pagination.total_pages, 5);
        assert!(pagination.has_next);
        assert!(pagination.has_previous);
    }

    #[test]
    fn test_sse_message_format() {
        let msg = SseMessage::event("update".to_string(), "{\"status\":\"ok\"}".to_string())
            .with_id("123".to_string());

        let formatted = msg.to_string();
        assert!(formatted.contains("event: update"));
        assert!(formatted.contains("id: 123"));
        assert!(formatted.contains("data: {\"status\":\"ok\"}"));
    }
}
