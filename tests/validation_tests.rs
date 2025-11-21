//! Integration tests for validation infrastructure
//!
//! These tests verify the validation framework and validators.

use llm_analytics_hub::infra::validation::types::*;

#[test]
fn test_validation_check_creation() {
    let check = ValidationCheck {
        name: "Test Check".to_string(),
        category: "Test Category".to_string(),
        status: CheckStatus::Passed,
        severity: CheckSeverity::Critical,
        message: "Test passed".to_string(),
        details: None,
    };

    assert_eq!(check.name, "Test Check");
    assert_eq!(check.status, CheckStatus::Passed);
    assert_eq!(check.severity, CheckSeverity::Critical);
}

#[test]
fn test_validation_check_with_details() {
    let check = ValidationCheck {
        name: "Detailed Check".to_string(),
        category: "Infrastructure".to_string(),
        status: CheckStatus::Failed,
        severity: CheckSeverity::Important,
        message: "Check failed".to_string(),
        details: Some("Additional failure information".to_string()),
    };

    assert!(check.details.is_some());
    assert_eq!(check.status, CheckStatus::Failed);
}

#[test]
fn test_validation_results_creation() {
    let results = ValidationResults {
        category: "Test Category".to_string(),
        checks: vec![
            ValidationCheck {
                name: "Check 1".to_string(),
                category: "Test".to_string(),
                status: CheckStatus::Passed,
                severity: CheckSeverity::Critical,
                message: "Passed".to_string(),
                details: None,
            },
            ValidationCheck {
                name: "Check 2".to_string(),
                category: "Test".to_string(),
                status: CheckStatus::Failed,
                severity: CheckSeverity::Important,
                message: "Failed".to_string(),
                details: Some("Reason".to_string()),
            },
        ],
        healthy: false,
    };

    assert_eq!(results.checks.len(), 2);
    assert!(!results.healthy);
}

#[test]
fn test_validation_report_structure() {
    let report = ValidationReport {
        timestamp: chrono::Utc::now().to_rfc3339(),
        environment: "test".to_string(),
        categories: vec![ValidationResults {
            category: "Cluster".to_string(),
            checks: vec![],
            healthy: true,
        }],
        healthy: true,
        total_checks: 0,
        passed_checks: 0,
        failed_checks: 0,
        warning_checks: 0,
        skipped_checks: 0,
        critical_failures: 0,
        important_failures: 0,
    };

    assert_eq!(report.environment, "test");
    assert!(report.healthy);
    assert_eq!(report.categories.len(), 1);
}

#[test]
fn test_check_status_variants() {
    let passed = CheckStatus::Passed;
    let failed = CheckStatus::Failed;
    let warning = CheckStatus::Warning;
    let skipped = CheckStatus::Skipped;

    assert_ne!(passed, failed);
    assert_ne!(failed, warning);
    assert_ne!(warning, skipped);
}

#[test]
fn test_check_severity_variants() {
    let critical = CheckSeverity::Critical;
    let important = CheckSeverity::Important;
    let advisory = CheckSeverity::Advisory;

    assert_ne!(critical, important);
    assert_ne!(important, advisory);
}

#[test]
fn test_validation_serialization() {
    let check = ValidationCheck {
        name: "Test".to_string(),
        category: "Test".to_string(),
        status: CheckStatus::Passed,
        severity: CheckSeverity::Critical,
        message: "OK".to_string(),
        details: None,
    };

    let json = serde_json::to_string(&check).unwrap();
    assert!(!json.is_empty());

    let deserialized: ValidationCheck = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, check.name);
    assert_eq!(deserialized.status, check.status);
}

#[test]
fn test_validation_report_serialization() {
    let report = ValidationReport {
        timestamp: "2025-11-20T12:00:00Z".to_string(),
        environment: "test".to_string(),
        categories: vec![],
        healthy: true,
        total_checks: 0,
        passed_checks: 0,
        failed_checks: 0,
        warning_checks: 0,
        skipped_checks: 0,
        critical_failures: 0,
        important_failures: 0,
    };

    let json = serde_json::to_string_pretty(&report).unwrap();
    assert!(!json.is_empty());
    assert!(json.contains("test"));
    assert!(json.contains("healthy"));
}

#[test]
fn test_validation_metrics_calculation() {
    let mut report = ValidationReport {
        timestamp: chrono::Utc::now().to_rfc3339(),
        environment: "test".to_string(),
        categories: vec![],
        healthy: true,
        total_checks: 5,
        passed_checks: 3,
        failed_checks: 1,
        warning_checks: 1,
        skipped_checks: 0,
        critical_failures: 1,
        important_failures: 0,
    };

    assert_eq!(report.total_checks, 5);
    assert_eq!(report.passed_checks, 3);
    assert_eq!(report.failed_checks, 1);
    assert_eq!(report.critical_failures, 1);

    // Health should be false if there are critical failures
    report.healthy = report.critical_failures == 0;
    assert!(!report.healthy);
}
