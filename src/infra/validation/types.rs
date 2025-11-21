//! Validation types and result structures

use serde::{Deserialize, Serialize};

/// Severity level for validation checks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckSeverity {
    /// Critical check - must pass
    Critical,
    /// Important check - should pass
    Important,
    /// Advisory check - nice to have
    Advisory,
}

/// Status of a validation check
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckStatus {
    /// Check passed
    Pass,
    /// Check failed
    Fail,
    /// Check passed with warnings
    Warn,
    /// Check was skipped
    Skip,
}

/// Individual validation check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCheck {
    /// Name of the check
    pub name: String,

    /// Category of the check
    pub category: String,

    /// Status of the check
    pub status: CheckStatus,

    /// Severity level
    pub severity: CheckSeverity,

    /// Detailed message
    pub message: String,

    /// Optional additional details
    pub details: Option<String>,
}

impl ValidationCheck {
    /// Create a new validation check
    pub fn new(name: impl Into<String>, category: impl Into<String>, severity: CheckSeverity) -> Self {
        Self {
            name: name.into(),
            category: category.into(),
            status: CheckStatus::Pass,
            severity,
            message: String::new(),
            details: None,
        }
    }

    /// Mark check as passed
    pub fn pass(mut self, message: impl Into<String>) -> Self {
        self.status = CheckStatus::Pass;
        self.message = message.into();
        self
    }

    /// Mark check as failed
    pub fn fail(mut self, message: impl Into<String>) -> Self {
        self.status = CheckStatus::Fail;
        self.message = message.into();
        self
    }

    /// Mark check as warning
    pub fn warn(mut self, message: impl Into<String>) -> Self {
        self.status = CheckStatus::Warn;
        self.message = message.into();
        self
    }

    /// Mark check as skipped
    pub fn skip(mut self, message: impl Into<String>) -> Self {
        self.status = CheckStatus::Skip;
        self.message = message.into();
        self
    }

    /// Add details to the check
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

/// Validation results for a category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResults {
    /// Category name
    pub category: String,

    /// Individual checks
    pub checks: Vec<ValidationCheck>,

    /// Overall status
    pub healthy: bool,

    /// Total checks run
    pub total: usize,

    /// Checks passed
    pub passed: usize,

    /// Checks failed
    pub failed: usize,

    /// Checks with warnings
    pub warnings: usize,
}

impl ValidationResults {
    /// Create new validation results
    pub fn new(category: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            checks: Vec::new(),
            healthy: true,
            total: 0,
            passed: 0,
            failed: 0,
            warnings: 0,
        }
    }

    /// Add a check result
    pub fn add_check(&mut self, check: ValidationCheck) {
        self.total += 1;

        match check.status {
            CheckStatus::Pass => self.passed += 1,
            CheckStatus::Fail => {
                self.failed += 1;
                if check.severity == CheckSeverity::Critical {
                    self.healthy = false;
                }
            }
            CheckStatus::Warn => self.warnings += 1,
            CheckStatus::Skip => {}
        }

        self.checks.push(check);
    }

    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            return 100.0;
        }
        (self.passed as f64 / self.total as f64) * 100.0
    }
}

/// Comprehensive validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Validation timestamp
    pub timestamp: String,

    /// Environment/namespace validated
    pub environment: String,

    /// Category results
    pub categories: Vec<ValidationResults>,

    /// Overall health status
    pub healthy: bool,

    /// Total checks across all categories
    pub total_checks: usize,

    /// Total passed
    pub total_passed: usize,

    /// Total failed
    pub total_failed: usize,

    /// Total warnings
    pub total_warnings: usize,
}

impl ValidationReport {
    /// Create new validation report
    pub fn new(environment: impl Into<String>) -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            environment: environment.into(),
            categories: Vec::new(),
            healthy: true,
            total_checks: 0,
            total_passed: 0,
            total_failed: 0,
            total_warnings: 0,
        }
    }

    /// Add category results
    pub fn add_category(&mut self, results: ValidationResults) {
        self.total_checks += results.total;
        self.total_passed += results.passed;
        self.total_failed += results.failed;
        self.total_warnings += results.warnings;

        if !results.healthy {
            self.healthy = false;
        }

        self.categories.push(results);
    }

    /// Get overall success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_checks == 0 {
            return 100.0;
        }
        (self.total_passed as f64 / self.total_checks as f64) * 100.0
    }
}
