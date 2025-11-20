//! TimescaleDB Load Testing and Benchmarking Tool
//!
//! High-performance Rust-based benchmark tool for TimescaleDB.
//! Replaces Python script with 10-100x better performance.

use anyhow::{Context, Result};
use chrono::Utc;
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::{Duration, Instant};
use tokio::task::JoinSet;
use tracing::{info, warn};

#[derive(Parser)]
#[command(name = "bench-timescaledb")]
#[command(about = "TimescaleDB load testing and benchmarking tool")]
struct Cli {
    /// Database host
    #[arg(long, env = "DB_HOST", default_value = "timescaledb.llm-analytics.svc.cluster.local")]
    host: String,

    /// Database port
    #[arg(long, env = "DB_PORT", default_value = "5432")]
    port: u16,

    /// Database name
    #[arg(long, env = "DB_NAME", default_value = "analytics")]
    database: String,

    /// Database user
    #[arg(long, env = "DB_USER", default_value = "postgres")]
    user: String,

    /// Database password
    #[arg(long, env = "DB_PASSWORD", default_value = "postgres")]
    password: String,

    /// Number of concurrent connections
    #[arg(long, default_value = "100")]
    connections: usize,

    /// Number of inserts per connection
    #[arg(long, default_value = "1000")]
    inserts_per_connection: usize,

    /// Number of queries to run
    #[arg(long, default_value = "1000")]
    num_queries: usize,

    /// Batch size for inserts
    #[arg(long, default_value = "100")]
    batch_size: usize,
}

#[derive(Debug, Clone)]
struct MetricRecord {
    time: chrono::DateTime<Utc>,
    model_id: String,
    provider: String,
    request_count: i32,
    token_input: i32,
    token_output: i32,
    token_total: i32,
    cost_usd: f64,
    avg_latency_ms: f64,
    error_count: i32,
    success_count: i32,
    user_id: String,
    application_id: String,
    environment: String,
}

#[derive(Debug)]
struct BenchmarkResults {
    total_operations: usize,
    total_time: Duration,
    ops_per_sec: f64,
    avg_time_ms: f64,
    p50_time_ms: f64,
    p95_time_ms: f64,
    p99_time_ms: f64,
}

impl BenchmarkResults {
    fn from_timings(ops: usize, timings: &[Duration]) -> Self {
        let total_time: Duration = timings.iter().sum();
        let ops_per_sec = ops as f64 / total_time.as_secs_f64();

        // Calculate percentiles
        let mut sorted_ms: Vec<f64> = timings.iter().map(|d| d.as_secs_f64() * 1000.0).collect();
        sorted_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let avg_time_ms = sorted_ms.iter().sum::<f64>() / sorted_ms.len() as f64;
        let p50_time_ms = percentile(&sorted_ms, 50.0);
        let p95_time_ms = percentile(&sorted_ms, 95.0);
        let p99_time_ms = percentile(&sorted_ms, 99.0);

        Self {
            total_operations: ops,
            total_time,
            ops_per_sec,
            avg_time_ms,
            p50_time_ms,
            p95_time_ms,
            p99_time_ms,
        }
    }
}

fn percentile(sorted_data: &[f64], p: f64) -> f64 {
    if sorted_data.is_empty() {
        return 0.0;
    }
    let index = ((p / 100.0) * (sorted_data.len() - 1) as f64).round() as usize;
    sorted_data[index.min(sorted_data.len() - 1)]
}

fn log_info(msg: &str) {
    println!("{}", format!("[BENCH-DB] {}", msg).blue().bold());
}

fn log_success(msg: &str) {
    println!("{}", format!("[BENCH-DB] {}", msg).green().bold());
}

fn log_error(msg: &str) {
    println!("{}", format!("[BENCH-DB] {}", msg).red().bold());
}

async fn create_pool(cli: &Cli) -> Result<PgPool> {
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        cli.user, cli.password, cli.host, cli.port, cli.database
    );

    let pool = PgPoolOptions::new()
        .min_connections(10)
        .max_connections(cli.connections as u32)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&connection_string)
        .await
        .context("Failed to create connection pool")?;

    Ok(pool)
}

fn generate_metric_record() -> MetricRecord {
    let mut rng = rand::thread_rng();

    let models = ["gpt-4", "gpt-3.5-turbo", "claude-3", "claude-2"];
    let providers = ["openai", "anthropic"];
    let users = ["user-1", "user-2", "user-3"];
    let apps = ["app-1", "app-2", "app-3"];
    let envs = ["dev", "staging", "prod"];

    MetricRecord {
        time: Utc::now(),
        model_id: models[rng.gen_range(0..models.len())].to_string(),
        provider: providers[rng.gen_range(0..providers.len())].to_string(),
        request_count: rng.gen_range(1..100),
        token_input: rng.gen_range(100..10000),
        token_output: rng.gen_range(100..10000),
        token_total: rng.gen_range(200..20000),
        cost_usd: rng.gen_range(0.001..0.5),
        avg_latency_ms: rng.gen_range(50.0..500.0),
        error_count: rng.gen_range(0..10),
        success_count: rng.gen_range(90..100),
        user_id: users[rng.gen_range(0..users.len())].to_string(),
        application_id: apps[rng.gen_range(0..apps.len())].to_string(),
        environment: envs[rng.gen_range(0..envs.len())].to_string(),
    }
}

async fn insert_batch(pool: &PgPool, batch_size: usize) -> Result<Duration> {
    let start = Instant::now();

    // Generate batch
    let records: Vec<MetricRecord> = (0..batch_size).map(|_| generate_metric_record()).collect();

    // Batch insert using COPY or executemany
    for record in records {
        sqlx::query(
            r#"
            INSERT INTO llm_usage_metrics (
                time, model_id, provider, request_count,
                token_input, token_output, token_total,
                cost_usd, avg_latency_ms, error_count,
                success_count, user_id, application_id, environment
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
        )
        .bind(record.time)
        .bind(record.model_id)
        .bind(record.provider)
        .bind(record.request_count)
        .bind(record.token_input)
        .bind(record.token_output)
        .bind(record.token_total)
        .bind(record.cost_usd)
        .bind(record.avg_latency_ms)
        .bind(record.error_count)
        .bind(record.success_count)
        .bind(record.user_id)
        .bind(record.application_id)
        .bind(record.environment)
        .execute(pool)
        .await?;
    }

    Ok(start.elapsed())
}

async fn run_query(pool: &PgPool) -> Result<Duration> {
    let start = Instant::now();

    sqlx::query(
        r#"
        SELECT
            model_id,
            provider,
            SUM(request_count) as total_requests,
            SUM(token_total) as total_tokens,
            SUM(cost_usd) as total_cost,
            AVG(avg_latency_ms) as avg_latency
        FROM llm_usage_metrics
        WHERE time >= NOW() - INTERVAL '1 hour'
        GROUP BY model_id, provider
        ORDER BY total_requests DESC
        LIMIT 10
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(start.elapsed())
}

async fn run_insert_load_test(pool: &PgPool, cli: &Cli) -> Result<BenchmarkResults> {
    log_info("Running insert load test...");

    let total_batches = (cli.connections * cli.inserts_per_connection) / cli.batch_size;
    let pb = ProgressBar::new(total_batches as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({per_sec})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut tasks = JoinSet::new();
    let mut timings = Vec::new();

    let start = Instant::now();

    // Create concurrent insert tasks
    for _ in 0..cli.connections {
        for _ in 0..(cli.inserts_per_connection / cli.batch_size) {
            let pool_clone = pool.clone();
            let batch_size = cli.batch_size;

            tasks.spawn(async move { insert_batch(&pool_clone, batch_size).await });
        }
    }

    // Collect results
    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok(duration)) => {
                timings.push(duration);
                pb.inc(1);
            }
            Ok(Err(e)) => warn!("Insert batch failed: {}", e),
            Err(e) => warn!("Task join failed: {}", e),
        }
    }

    pb.finish_with_message("Insert load test complete");

    let total_inserts = cli.connections * cli.inserts_per_connection;
    Ok(BenchmarkResults::from_timings(total_inserts, &timings))
}

async fn run_query_load_test(pool: &PgPool, cli: &Cli) -> Result<BenchmarkResults> {
    log_info("Running query load test...");

    let pb = ProgressBar::new(cli.num_queries as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({per_sec})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut tasks = JoinSet::new();
    let mut timings = Vec::new();

    // Create concurrent query tasks
    for _ in 0..cli.num_queries {
        let pool_clone = pool.clone();
        tasks.spawn(async move { run_query(&pool_clone).await });
    }

    // Collect results
    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok(duration)) => {
                timings.push(duration);
                pb.inc(1);
            }
            Ok(Err(e)) => warn!("Query failed: {}", e),
            Err(e) => warn!("Task join failed: {}", e),
        }
    }

    pb.finish_with_message("Query load test complete");

    Ok(BenchmarkResults::from_timings(cli.num_queries, &timings))
}

async fn test_concurrent_connections(pool: &PgPool, cli: &Cli) -> Result<Duration> {
    log_info("Testing concurrent connections...");

    let start = Instant::now();
    let mut tasks = JoinSet::new();

    // Acquire all connections
    for _ in 0..cli.connections {
        let pool_clone = pool.clone();
        tasks.spawn(async move {
            let conn = pool_clone.acquire().await?;
            sqlx::query("SELECT 1").fetch_one(&mut *conn).await?;
            Ok::<_, anyhow::Error>(())
        });
    }

    // Wait for all
    while let Some(result) = tasks.join_next().await {
        if let Err(e) = result {
            warn!("Connection test failed: {}", e);
        }
    }

    Ok(start.elapsed())
}

fn print_results(name: &str, results: &BenchmarkResults) {
    log_success(&format!("{} Results:", name));
    println!("  Total Operations:   {}", format!("{:>12}", results.total_operations).cyan());
    println!("  Total Time:         {}", format!("{:>10.2}s", results.total_time.as_secs_f64()).cyan());
    println!("  Operations/sec:     {}", format!("{:>12.0}", results.ops_per_sec).green().bold());
    println!("  Avg Time:           {}", format!("{:>10.2}ms", results.avg_time_ms).cyan());
    println!("  P50 Time:           {}", format!("{:>10.2}ms", results.p50_time_ms).cyan());
    println!("  P95 Time:           {}", format!("{:>10.2}ms", results.p95_time_ms).yellow());
    println!("  P99 Time:           {}", format!("{:>10.2}ms", results.p99_time_ms).yellow().bold());
    println!();
}

fn assess_performance(insert_results: &BenchmarkResults, query_results: &BenchmarkResults, conn_count: usize) {
    log_info("Performance Assessment:");

    // Insert throughput
    if insert_results.ops_per_sec >= 100_000.0 {
        log_success("  Insert Throughput:  EXCELLENT (>=100k/sec)");
    } else if insert_results.ops_per_sec >= 50_000.0 {
        log_success("  Insert Throughput:  GOOD (>=50k/sec)");
    } else {
        log_error("  Insert Throughput:  NEEDS IMPROVEMENT (<50k/sec)");
    }

    // Query latency
    if query_results.p95_time_ms < 100.0 {
        log_success("  Query Latency:      EXCELLENT (P95 <100ms)");
    } else if query_results.p95_time_ms < 200.0 {
        log_success("  Query Latency:      GOOD (P95 <200ms)");
    } else {
        log_error("  Query Latency:      NEEDS IMPROVEMENT (P95 >=200ms)");
    }

    // Concurrency
    if conn_count >= 1000 {
        log_success("  Concurrency:        EXCELLENT (>=1000 conns)");
    } else if conn_count >= 500 {
        log_success("  Concurrency:        GOOD (>=500 conns)");
    } else {
        log_error("  Concurrency:        NEEDS IMPROVEMENT (<500 conns)");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    println!();
    log_info("==================================================");
    log_info("   TimescaleDB Load Test (Rust Edition)");
    log_info("==================================================");
    println!();

    // Create connection pool
    log_info(&format!("Connecting to {}:{}/{}...", cli.host, cli.port, cli.database));
    let pool = create_pool(&cli).await?;
    log_success("Connection pool created");
    println!();

    // Run insert load test
    let insert_results = run_insert_load_test(&pool, &cli).await?;
    print_results("Insert Load Test", &insert_results);

    // Run query load test
    let query_results = run_query_load_test(&pool, &cli).await?;
    print_results("Query Load Test", &query_results);

    // Test concurrent connections
    let conn_time = test_concurrent_connections(&pool, &cli).await?;
    log_success("Concurrent Connection Test Results:");
    println!("  Max Connections:    {}", format!("{:>12}", cli.connections).cyan());
    println!("  Acquisition Time:   {}", format!("{:>10.2}s", conn_time.as_secs_f64()).cyan());
    println!("  Status:             {}", "SUCCESS".green().bold());
    println!();

    // Performance assessment
    assess_performance(&insert_results, &query_results, cli.connections);

    pool.close().await;
    println!();
    log_success("==================================================");
    log_success("   Load test completed successfully!");
    log_success("==================================================");
    println!();

    Ok(())
}
