# Testing Documentation

## Overview

This document provides comprehensive testing guidelines for the LLM Analytics Hub project. The project follows enterprise-grade testing practices with multiple levels of testing to ensure production readiness.

## Test Organization

### Test Structure

```
llm-analytics-hub/
├── tests/                          # Integration tests
│   ├── k8s_operations_tests.rs     # K8s client tests
│   ├── validation_tests.rs         # Validation framework tests
│   ├── backup_restore_tests.rs     # Backup/restore tests
│   ├── kafka_redis_tests.rs        # Kafka/Redis tests
│   ├── property_tests.rs           # Property-based tests
│   ├── integration_event_pipeline.rs
│   └── security_tests.rs
├── benches/                        # Performance benchmarks
│   ├── infrastructure_benchmarks.rs
│   ├── event_processing.rs
│   ├── metric_aggregation.rs
│   └── timeseries_query.rs
└── src/                            # Unit tests (in modules)
    ├── infra/
    │   ├── k8s/client.rs          # Unit tests at bottom
    │   ├── validation/types.rs     # Unit tests at bottom
    │   └── ...
    └── ...
```

## Test Categories

### 1. Unit Tests

Unit tests are located within each module file using the `#[cfg(test)]` attribute.

**Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        let result = some_function();
        assert_eq!(result, expected_value);
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

**Running unit tests:**
```bash
# Run all unit tests
cargo test --lib

# Run tests for specific module
cargo test --lib k8s::client

# Run with output
cargo test --lib -- --nocapture
```

### 2. Integration Tests

Integration tests are in the `tests/` directory and test multiple components together.

**Key integration test files:**
- `k8s_operations_tests.rs` - Kubernetes operations
- `validation_tests.rs` - Validation framework
- `backup_restore_tests.rs` - Backup and restore
- `kafka_redis_tests.rs` - Message queue and cache
- `integration_event_pipeline.rs` - End-to-end event processing

**Running integration tests:**
```bash
# Run all integration tests
cargo test --test '*'

# Run specific test file
cargo test --test validation_tests

# Run specific test
cargo test --test validation_tests test_validation_check_creation
```

### 3. Property-Based Tests

Property-based tests use `proptest` to verify properties hold for all inputs.

**Located in:** `tests/property_tests.rs`

**Example:**
```rust
proptest! {
    #[test]
    fn test_backup_size_always_non_negative(size in 0u64..1_000_000_000u64) {
        let mut metadata = BackupMetadata::new("backup-123", "test_db");
        metadata.complete(size, "s3://bucket/backup");
        assert!(metadata.size_bytes >= 0);
    }
}
```

**Running property tests:**
```bash
cargo test --test property_tests
```

### 4. Documentation Tests

Doc tests are embedded in documentation comments and verify examples work.

**Example:**
```rust
/// Create a new backup configuration
///
/// # Examples
/// ```
/// use llm_analytics_hub::infra::backup::BackupConfig;
///
/// let config = BackupConfig::default();
/// assert_eq!(config.retention_days, 30);
/// ```
pub fn create_config() -> BackupConfig {
    BackupConfig::default()
}
```

**Running doc tests:**
```bash
cargo test --doc
```

### 5. Benchmarks

Performance benchmarks measure critical path performance.

**Located in:** `benches/infrastructure_benchmarks.rs`

**Running benchmarks:**
```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench backup_metadata_creation

# Save baseline
cargo bench --bench infrastructure_benchmarks -- --save-baseline main

# Compare to baseline
cargo bench --bench infrastructure_benchmarks -- --baseline main
```

## Test Execution

### Running All Tests

```bash
# Run everything
cargo test --all-features

# Run with verbose output
cargo test --all-features -- --nocapture

# Run with specific log level
RUST_LOG=debug cargo test --all-features
```

### Running Specific Test Types

```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Doc tests only
cargo test --doc

# Specific test by name
cargo test test_backup_metadata_creation

# Tests matching pattern
cargo test backup
```

### Running with Different Features

```bash
# With all features
cargo test --all-features

# With specific features
cargo test --features "ml,telemetry"

# Without default features
cargo test --no-default-features
```

### Ignoring Slow Tests

Some tests require external resources (K8s cluster, databases) and are marked with `#[ignore]`.

```bash
# Run only fast tests
cargo test

# Run ignored tests
cargo test -- --ignored

# Run all tests (including ignored)
cargo test -- --include-ignored
```

## Code Coverage

### Using Tarpaulin

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir target/coverage

# Generate with specific features
cargo tarpaulin --all-features --out Html

# View coverage
open target/coverage/index.html
```

### Coverage Goals

- **Overall**: > 70%
- **Critical modules**: > 80%
- **New code**: > 85%

## Linting and Formatting

### Clippy (Linter)

```bash
# Run clippy
cargo clippy --all-features --all-targets

# Treat warnings as errors
cargo clippy --all-features --all-targets -- -D warnings

# Fix automatically where possible
cargo clippy --fix --all-features
```

### Rustfmt (Formatter)

```bash
# Check formatting
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all
```

## CI/CD Integration

Tests run automatically on:
- Every push to `main` or `develop`
- Every pull request

**GitHub Actions workflow:** `.github/workflows/rust-tests.yml`

**Jobs:**
1. **Test Suite** - Runs all tests on stable and beta Rust
2. **Clippy** - Linting checks
3. **Rustfmt** - Formatting checks
4. **Coverage** - Code coverage reporting
5. **Benchmarks** - Performance regression detection
6. **Security Audit** - Dependency vulnerability scanning
7. **Build Check** - Cross-platform builds

## Writing Good Tests

### Unit Test Guidelines

1. **Test one thing per test**
   ```rust
   #[test]
   fn test_backup_metadata_creation() {
       let metadata = BackupMetadata::new("id", "db");
       assert_eq!(metadata.backup_id, "id");
   }
   ```

2. **Use descriptive names**
   - Good: `test_backup_metadata_creation_with_valid_id`
   - Bad: `test1`

3. **Arrange-Act-Assert pattern**
   ```rust
   #[test]
   fn test_example() {
       // Arrange
       let input = create_test_data();

       // Act
       let result = function_under_test(input);

       // Assert
       assert_eq!(result, expected);
   }
   ```

4. **Test error cases**
   ```rust
   #[test]
   fn test_invalid_input_returns_error() {
       let result = function("");
       assert!(result.is_err());
   }
   ```

### Integration Test Guidelines

1. **Test realistic scenarios**
2. **Clean up resources**
3. **Use fixtures for test data**
4. **Mock external dependencies when possible**

### Property Test Guidelines

1. **Define input constraints**
   ```rust
   proptest! {
       #[test]
       fn test_property(value in 1..100) {
           assert!(property_holds(value));
       }
   }
   ```

2. **Test invariants**
3. **Use appropriate generators**

## Test Data and Fixtures

### Creating Test Data

```rust
// Helper functions for test data
fn create_test_backup_config() -> BackupConfig {
    BackupConfig {
        s3_bucket: "test-bucket".to_string(),
        s3_prefix: "test".to_string(),
        aws_region: "us-east-1".to_string(),
        encryption: true,
        compression: true,
        retention_days: 7,
    }
}
```

### Using Fake Data

```rust
use fake::{Fake, Faker};

#[test]
fn test_with_fake_data() {
    let name: String = Faker.fake();
    let result = process_name(&name);
    assert!(result.is_ok());
}
```

## Mocking and Stubbing

### Using Mockall

```rust
use mockall::*;

#[automock]
trait Database {
    fn query(&self, sql: &str) -> Result<Vec<String>>;
}

#[test]
fn test_with_mock() {
    let mut mock = MockDatabase::new();
    mock.expect_query()
        .returning(|_| Ok(vec!["result".to_string()]));

    let result = process_database(&mock);
    assert!(result.is_ok());
}
```

## Performance Testing

### Benchmark Best Practices

1. **Use `black_box` to prevent optimization**
   ```rust
   b.iter(|| {
       let result = function(black_box(input));
       black_box(result)
   });
   ```

2. **Parameterize benchmarks**
   ```rust
   for size in [10, 100, 1000].iter() {
       group.bench_with_input(
           BenchmarkId::from_parameter(size),
           size,
           |b, &s| b.iter(|| process(s))
       );
   }
   ```

3. **Establish baselines**
4. **Monitor regressions**

## Debugging Tests

### Running with Debug Output

```bash
# Show println! output
cargo test -- --nocapture

# Show test names
cargo test -- --test-threads=1 --nocapture

# Run specific test with logs
RUST_LOG=debug cargo test test_name -- --nocapture
```

### Using Test Filters

```bash
# Run tests matching pattern
cargo test backup

# Run tests in specific module
cargo test infra::k8s

# Exclude tests
cargo test -- --skip slow_test
```

## Test Metrics

### Current Coverage

| Module | Coverage | Target |
|--------|----------|--------|
| infra/k8s | 65% | 75% |
| infra/backup | 70% | 80% |
| infra/validation | 75% | 80% |
| infra/kafka | 68% | 75% |
| infra/redis | 68% | 75% |
| cli/* | 60% | 70% |

### Performance Baselines

| Operation | Baseline | Threshold |
|-----------|----------|-----------|
| Backup metadata creation | 120ns | 200ns |
| Topic config creation | 150ns | 250ns |
| Validation check creation | 100ns | 180ns |
| LLM topics generation | 2.5µs | 5µs |

## Troubleshooting

### Common Issues

**Issue:** Tests fail with "connection refused"
- **Solution:** Tests requiring K8s/DB are marked `#[ignore]`, skip them with `cargo test`

**Issue:** Slow test execution
- **Solution:** Run with `--test-threads=N` to parallelize

**Issue:** Flaky tests
- **Solution:** Identify timing issues, add proper synchronization

**Issue:** Coverage tool fails
- **Solution:** Ensure tarpaulin is installed and run with `--all-features`

## Best Practices Summary

✅ **DO:**
- Write tests for all public APIs
- Test error cases
- Use descriptive test names
- Keep tests focused and isolated
- Mock external dependencies
- Run tests before committing
- Aim for > 70% coverage

❌ **DON'T:**
- Write tests that depend on each other
- Use hardcoded values (use constants/fixtures)
- Skip error handling in tests
- Leave commented-out tests
- Write tests that require manual setup
- Commit failing tests

## Resources

- [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Proptest Documentation](https://docs.rs/proptest/)
- [Criterion Documentation](https://docs.rs/criterion/)
- [Mockall Documentation](https://docs.rs/mockall/)
- [Tarpaulin Documentation](https://github.com/xd009642/tarpaulin)

## Contributing

When adding new features:
1. Write tests first (TDD when appropriate)
2. Ensure all tests pass
3. Check coverage hasn't decreased
4. Run clippy and fix warnings
5. Format code with rustfmt
6. Update this documentation if needed

---

**Last Updated:** 2025-11-20
**Maintainers:** LLM Analytics Team
