//! Benchmarks for infrastructure operations
//!
//! These benchmarks measure performance of critical paths in the infrastructure code.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use llm_analytics_hub::infra::backup::types::*;
use llm_analytics_hub::infra::kafka::types::*;
use llm_analytics_hub::infra::validation::types::*;

// ============================================================================
// BACKUP OPERATIONS BENCHMARKS
// ============================================================================

fn benchmark_backup_metadata_creation(c: &mut Criterion) {
    c.bench_function("backup_metadata_creation", |b| {
        b.iter(|| {
            let metadata = BackupMetadata::new(
                black_box("backup-12345"),
                black_box("test_database"),
            );
            black_box(metadata)
        })
    });
}

fn benchmark_backup_metadata_completion(c: &mut Criterion) {
    c.bench_function("backup_metadata_completion", |b| {
        b.iter(|| {
            let mut metadata = BackupMetadata::new("backup-12345", "test_database");
            metadata.complete(
                black_box(1024000),
                black_box("s3://bucket/backup-12345.tar.gz"),
            );
            black_box(metadata)
        })
    });
}

fn benchmark_backup_serialization(c: &mut Criterion) {
    let metadata = BackupMetadata::new("backup-12345", "test_database");

    c.bench_function("backup_metadata_serialization", |b| {
        b.iter(|| {
            let json = serde_json::to_string(&metadata).unwrap();
            black_box(json)
        })
    });
}

fn benchmark_backup_deserialization(c: &mut Criterion) {
    let metadata = BackupMetadata::new("backup-12345", "test_database");
    let json = serde_json::to_string(&metadata).unwrap();

    c.bench_function("backup_metadata_deserialization", |b| {
        b.iter(|| {
            let metadata: BackupMetadata = serde_json::from_str(black_box(&json)).unwrap();
            black_box(metadata)
        })
    });
}

// ============================================================================
// KAFKA OPERATIONS BENCHMARKS
// ============================================================================

fn benchmark_topic_config_creation(c: &mut Criterion) {
    c.bench_function("topic_config_creation", |b| {
        b.iter(|| {
            let config = TopicConfig::new(
                black_box("test-topic"),
                black_box(32),
                black_box(3),
                black_box("Test topic"),
            );
            black_box(config)
        })
    });
}

fn benchmark_topic_config_with_settings(c: &mut Criterion) {
    c.bench_function("topic_config_with_settings", |b| {
        b.iter(|| {
            let config = TopicConfig::new("test-topic", 32, 3, "Test")
                .with_retention_ms(black_box(604_800_000))
                .with_compression(black_box("lz4"))
                .with_min_isr(black_box(2));
            black_box(config)
        })
    });
}

fn benchmark_llm_topics_generation(c: &mut Criterion) {
    c.bench_function("llm_topics_generation", |b| {
        b.iter(|| {
            let configs = get_llm_topic_configs();
            black_box(configs)
        })
    });
}

fn benchmark_standard_acls_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("acls_generation");

    group.bench_function("producer_acls", |b| {
        b.iter(|| {
            let acls = get_standard_producer_acls();
            black_box(acls)
        })
    });

    group.bench_function("consumer_acls", |b| {
        b.iter(|| {
            let acls = get_standard_consumer_acls();
            black_box(acls)
        })
    });

    group.finish();
}

fn benchmark_topic_serialization(c: &mut Criterion) {
    let config = TopicConfig::new("test-topic", 32, 3, "Test")
        .with_compression("lz4");

    c.bench_function("topic_config_serialization", |b| {
        b.iter(|| {
            let json = serde_json::to_string(&config).unwrap();
            black_box(json)
        })
    });
}

// ============================================================================
// VALIDATION OPERATIONS BENCHMARKS
// ============================================================================

fn benchmark_validation_check_creation(c: &mut Criterion) {
    c.bench_function("validation_check_creation", |b| {
        b.iter(|| {
            let check = ValidationCheck {
                name: black_box("Test Check").to_string(),
                category: black_box("Category").to_string(),
                status: CheckStatus::Passed,
                severity: CheckSeverity::Critical,
                message: black_box("Check passed").to_string(),
                details: None,
            };
            black_box(check)
        })
    });
}

fn benchmark_validation_report_creation(c: &mut Criterion) {
    c.bench_function("validation_report_creation", |b| {
        b.iter(|| {
            let report = ValidationReport {
                timestamp: chrono::Utc::now().to_rfc3339(),
                environment: black_box("test").to_string(),
                categories: vec![],
                healthy: true,
                total_checks: black_box(100),
                passed_checks: black_box(95),
                failed_checks: black_box(5),
                warning_checks: black_box(0),
                skipped_checks: black_box(0),
                critical_failures: black_box(0),
                important_failures: black_box(2),
            };
            black_box(report)
        })
    });
}

fn benchmark_validation_serialization(c: &mut Criterion) {
    let check = ValidationCheck {
        name: "Test Check".to_string(),
        category: "Category".to_string(),
        status: CheckStatus::Passed,
        severity: CheckSeverity::Critical,
        message: "Check passed".to_string(),
        details: None,
    };

    c.bench_function("validation_check_serialization", |b| {
        b.iter(|| {
            let json = serde_json::to_string(&check).unwrap();
            black_box(json)
        })
    });
}

// ============================================================================
// SCALABILITY BENCHMARKS
// ============================================================================

fn benchmark_large_validation_report(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_validation_report");

    for num_checks in [10, 50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(num_checks), num_checks, |b, &n| {
            b.iter(|| {
                let checks: Vec<ValidationCheck> = (0..n)
                    .map(|i| ValidationCheck {
                        name: format!("Check {}", i),
                        category: "Test".to_string(),
                        status: CheckStatus::Passed,
                        severity: CheckSeverity::Critical,
                        message: "OK".to_string(),
                        details: None,
                    })
                    .collect();

                let results = ValidationResults {
                    category: "Test".to_string(),
                    checks,
                    healthy: true,
                };

                black_box(results)
            })
        });
    }

    group.finish();
}

fn benchmark_multiple_topics_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_topics_serialization");

    for num_topics in [1, 5, 14, 50].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(num_topics), num_topics, |b, &n| {
            let configs: Vec<TopicConfig> = (0..n)
                .map(|i| TopicConfig::new(&format!("topic-{}", i), 32, 3, "Test"))
                .collect();

            b.iter(|| {
                let json = serde_json::to_string(&configs).unwrap();
                black_box(json)
            })
        });
    }

    group.finish();
}

// ============================================================================
// BENCHMARK GROUPS
// ============================================================================

criterion_group!(
    backup_benches,
    benchmark_backup_metadata_creation,
    benchmark_backup_metadata_completion,
    benchmark_backup_serialization,
    benchmark_backup_deserialization
);

criterion_group!(
    kafka_benches,
    benchmark_topic_config_creation,
    benchmark_topic_config_with_settings,
    benchmark_llm_topics_generation,
    benchmark_standard_acls_generation,
    benchmark_topic_serialization
);

criterion_group!(
    validation_benches,
    benchmark_validation_check_creation,
    benchmark_validation_report_creation,
    benchmark_validation_serialization
);

criterion_group!(
    scalability_benches,
    benchmark_large_validation_report,
    benchmark_multiple_topics_serialization
);

criterion_main!(
    backup_benches,
    kafka_benches,
    validation_benches,
    scalability_benches
);
