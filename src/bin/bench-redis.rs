//! Redis Load Testing and Benchmarking Tool
//!
//! High-performance Rust-based benchmark tool for Redis.
//! Replaces Python script with 10-100x better performance.

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client};
use std::time::{Duration, Instant};
use tokio::task::JoinSet;
use tracing::{info, warn};

#[derive(Parser)]
#[command(name = "bench-redis")]
#[command(about = "Redis load testing and benchmarking tool")]
struct Cli {
    /// Redis host
    #[arg(long, env = "REDIS_HOST", default_value = "redis-master.llm-analytics.svc.cluster.local")]
    host: String,

    /// Redis port
    #[arg(long, env = "REDIS_PORT", default_value = "6379")]
    port: u16,

    /// Number of concurrent connections
    #[arg(long, default_value = "100")]
    connections: usize,

    /// Number of operations to run
    #[arg(long, default_value = "100000")]
    num_operations: usize,

    /// Number of cache keys
    #[arg(long, default_value = "10000")]
    cache_keys: usize,
}

#[derive(Debug)]
struct BenchmarkResults {
    operation: String,
    total_operations: usize,
    total_time: Duration,
    ops_per_sec: f64,
    avg_time_ms: f64,
    p50_time_ms: f64,
    p95_time_ms: f64,
    p99_time_ms: f64,
    hit_ratio: Option<f64>,
}

impl BenchmarkResults {
    fn from_timings(operation: &str, ops: usize, timings: &[Duration], hit_ratio: Option<f64>) -> Self {
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
            operation: operation.to_string(),
            total_operations: ops,
            total_time,
            ops_per_sec,
            avg_time_ms,
            p50_time_ms,
            p95_time_ms,
            p99_time_ms,
            hit_ratio,
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
    println!("{}", format!("[REDIS-BENCH] {}", msg).blue().bold());
}

fn log_success(msg: &str) {
    println!("{}", format!("[REDIS-BENCH] {}", msg).green().bold());
}

fn log_error(msg: &str) {
    println!("{}", format!("[REDIS-BENCH] {}", msg).red().bold());
}

async fn create_client(cli: &Cli) -> Result<Client> {
    let connection_string = format!("redis://{}:{}", cli.host, cli.port);
    let client = Client::open(connection_string).context("Failed to create Redis client")?;
    Ok(client)
}

async fn run_set_operations(
    mut conn: ConnectionManager,
    num_ops: usize,
    cache_keys: usize,
) -> Result<Duration> {
    let start = Instant::now();
    let mut rng = rand::thread_rng();

    for i in 0..num_ops {
        let key = format!("load_test:key:{}", rng.gen_range(0..cache_keys));
        let value = format!("value_{}_{}", i, rng.gen_range(0..1_000_000));
        conn.set_ex::<_, _, ()>(&key, &value, 300).await?;
    }

    Ok(start.elapsed())
}

async fn run_get_operations(
    mut conn: ConnectionManager,
    num_ops: usize,
    cache_keys: usize,
) -> Result<(Duration, f64)> {
    let start = Instant::now();
    let mut rng = rand::thread_rng();
    let mut hits = 0;

    for _ in 0..num_ops {
        let key = format!("load_test:key:{}", rng.gen_range(0..cache_keys));
        let value: Option<String> = conn.get(&key).await?;
        if value.is_some() {
            hits += 1;
        }
    }

    let elapsed = start.elapsed();
    let hit_ratio = hits as f64 / num_ops as f64;

    Ok((elapsed, hit_ratio))
}

async fn run_mixed_operations(
    mut conn: ConnectionManager,
    num_ops: usize,
    cache_keys: usize,
) -> Result<Duration> {
    let start = Instant::now();
    let mut rng = rand::thread_rng();

    for i in 0..num_ops {
        let op = rng.gen_range(0..5);

        match op {
            0 => {
                // SET
                let key = format!("load_test:mixed:{}", rng.gen_range(0..cache_keys));
                let value = format!("value_{}", i);
                conn.set_ex::<_, _, ()>(&key, &value, 300).await?;
            }
            1 => {
                // GET
                let key = format!("load_test:mixed:{}", rng.gen_range(0..cache_keys));
                let _: Option<String> = conn.get(&key).await?;
            }
            2 => {
                // INCR
                let key = format!("load_test:counter:{}", rng.gen_range(0..100));
                conn.incr::<_, _, ()>(&key, 1).await?;
            }
            3 => {
                // LPUSH
                let key = format!("load_test:list:{}", rng.gen_range(0..100));
                let value = format!("item_{}", i);
                conn.lpush::<_, _, ()>(&key, &value).await?;
            }
            4 => {
                // HSET
                let key = format!("load_test:hash:{}", rng.gen_range(0..100));
                let field = format!("field_{}", i);
                let value = format!("value_{}", i);
                conn.hset::<_, _, _, ()>(&key, &field, &value).await?;
            }
            _ => unreachable!(),
        }
    }

    Ok(start.elapsed())
}

async fn run_set_load_test(client: &Client, cli: &Cli) -> Result<BenchmarkResults> {
    log_info("Running SET operations load test...");

    let ops_per_task = cli.num_operations / cli.connections;
    let pb = ProgressBar::new(cli.connections as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({per_sec})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut tasks = JoinSet::new();
    let mut timings = Vec::new();

    // Create concurrent tasks
    for _ in 0..cli.connections {
        let client_clone = client.clone();
        let cache_keys = cli.cache_keys;

        tasks.spawn(async move {
            let conn = ConnectionManager::new(client_clone).await?;
            run_set_operations(conn, ops_per_task, cache_keys).await
        });
    }

    // Collect results
    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok(duration)) => {
                timings.push(duration);
                pb.inc(1);
            }
            Ok(Err(e)) => warn!("SET operation failed: {}", e),
            Err(e) => warn!("Task join failed: {}", e),
        }
    }

    pb.finish_with_message("SET load test complete");

    Ok(BenchmarkResults::from_timings(
        "SET",
        cli.num_operations,
        &timings,
        None,
    ))
}

async fn prepopulate_cache(client: &Client, cache_keys: usize) -> Result<()> {
    log_info(&format!("Pre-populating cache with {} keys...", cache_keys));

    let mut conn = ConnectionManager::new(client.clone()).await?;

    for i in 0..cache_keys {
        let key = format!("load_test:key:{}", i);
        let value = format!("value_{}", i);
        conn.set_ex::<_, _, ()>(&key, &value, 600).await?;
    }

    log_success("Cache pre-populated");
    Ok(())
}

async fn run_get_load_test(client: &Client, cli: &Cli) -> Result<BenchmarkResults> {
    log_info("Running GET operations load test...");

    // Pre-populate cache
    prepopulate_cache(client, cli.cache_keys).await?;

    let ops_per_task = cli.num_operations / cli.connections;
    let pb = ProgressBar::new(cli.connections as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({per_sec})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut tasks = JoinSet::new();
    let mut timings = Vec::new();
    let mut hit_ratios = Vec::new();

    // Create concurrent tasks
    for _ in 0..cli.connections {
        let client_clone = client.clone();
        let cache_keys = cli.cache_keys;

        tasks.spawn(async move {
            let conn = ConnectionManager::new(client_clone).await?;
            run_get_operations(conn, ops_per_task, cache_keys).await
        });
    }

    // Collect results
    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok((duration, hit_ratio))) => {
                timings.push(duration);
                hit_ratios.push(hit_ratio);
                pb.inc(1);
            }
            Ok(Err(e)) => warn!("GET operation failed: {}", e),
            Err(e) => warn!("Task join failed: {}", e),
        }
    }

    pb.finish_with_message("GET load test complete");

    let avg_hit_ratio = hit_ratios.iter().sum::<f64>() / hit_ratios.len() as f64;

    Ok(BenchmarkResults::from_timings(
        "GET",
        cli.num_operations,
        &timings,
        Some(avg_hit_ratio),
    ))
}

async fn run_mixed_load_test(client: &Client, cli: &Cli) -> Result<BenchmarkResults> {
    log_info("Running mixed operations load test...");

    let ops_per_task = cli.num_operations / cli.connections;
    let pb = ProgressBar::new(cli.connections as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({per_sec})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut tasks = JoinSet::new();
    let mut timings = Vec::new();

    // Create concurrent tasks
    for _ in 0..cli.connections {
        let client_clone = client.clone();
        let cache_keys = cli.cache_keys;

        tasks.spawn(async move {
            let conn = ConnectionManager::new(client_clone).await?;
            run_mixed_operations(conn, ops_per_task, cache_keys).await
        });
    }

    // Collect results
    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok(duration)) => {
                timings.push(duration);
                pb.inc(1);
            }
            Ok(Err(e)) => warn!("Mixed operation failed: {}", e),
            Err(e) => warn!("Task join failed: {}", e),
        }
    }

    pb.finish_with_message("Mixed operations load test complete");

    Ok(BenchmarkResults::from_timings(
        "MIXED",
        cli.num_operations,
        &timings,
        None,
    ))
}

async fn cleanup_test_data(client: &Client, cache_keys: usize) -> Result<()> {
    log_info("Cleaning up test data...");

    let mut conn = ConnectionManager::new(client.clone()).await?;

    for i in 0..cache_keys {
        let key = format!("load_test:key:{}", i);
        conn.del::<_, ()>(&key).await.ok();
    }

    log_success("Test data cleaned up");
    Ok(())
}

fn print_results(results: &BenchmarkResults) {
    log_success(&format!("{} Load Test Results:", results.operation));
    println!("  Total Operations:   {}", format!("{:>12}", results.total_operations).cyan());
    println!("  Total Time:         {}", format!("{:>10.2}s", results.total_time.as_secs_f64()).cyan());
    println!("  Operations/sec:     {}", format!("{:>12.0}", results.ops_per_sec).green().bold());
    println!("  Avg Time:           {}", format!("{:>10.2}ms", results.avg_time_ms).cyan());
    println!("  P50 Time:           {}", format!("{:>10.2}ms", results.p50_time_ms).cyan());
    println!("  P95 Time:           {}", format!("{:>10.2}ms", results.p95_time_ms).yellow());
    println!("  P99 Time:           {}", format!("{:>10.2}ms", results.p99_time_ms).yellow().bold());

    if let Some(hit_ratio) = results.hit_ratio {
        println!("  Cache Hit Ratio:    {}", format!("{:>10.1}%", hit_ratio * 100.0).green().bold());
    }

    println!();
}

fn assess_performance(set_results: &BenchmarkResults, get_results: &BenchmarkResults, mixed_results: &BenchmarkResults) {
    log_info("Performance Assessment:");

    let avg_ops = (set_results.ops_per_sec + get_results.ops_per_sec + mixed_results.ops_per_sec) / 3.0;

    // Throughput
    if avg_ops >= 100_000.0 {
        log_success("  Throughput:         EXCELLENT (>=100k ops/sec)");
    } else if avg_ops >= 50_000.0 {
        log_success("  Throughput:         GOOD (>=50k ops/sec)");
    } else {
        log_error("  Throughput:         NEEDS IMPROVEMENT (<50k ops/sec)");
    }

    // Cache hit ratio
    if let Some(hit_ratio) = get_results.hit_ratio {
        if hit_ratio >= 0.90 {
            log_success("  Cache Hit Ratio:    EXCELLENT (>=90%)");
        } else if hit_ratio >= 0.70 {
            log_success("  Cache Hit Ratio:    GOOD (>=70%)");
        } else {
            log_error("  Cache Hit Ratio:    NEEDS IMPROVEMENT (<70%)");
        }
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
    log_info("      Redis Load Test (Rust Edition)");
    log_info("==================================================");
    println!();

    // Create Redis client
    log_info(&format!("Connecting to {}:{}...", cli.host, cli.port));
    let client = create_client(&cli).await?;

    // Test connection
    let mut conn = ConnectionManager::new(client.clone()).await?;
    redis::cmd("PING").query_async::<_, String>(&mut conn).await?;
    log_success("Connected to Redis");
    println!();

    // Run SET load test
    let set_results = run_set_load_test(&client, &cli).await?;
    print_results(&set_results);

    // Run GET load test
    let get_results = run_get_load_test(&client, &cli).await?;
    print_results(&get_results);

    // Run mixed load test
    let mixed_results = run_mixed_load_test(&client, &cli).await?;
    print_results(&mixed_results);

    // Performance assessment
    assess_performance(&set_results, &get_results, &mixed_results);

    // Cleanup
    cleanup_test_data(&client, cli.cache_keys).await?;

    println!();
    log_success("==================================================");
    log_success("   Load test completed successfully!");
    log_success("==================================================");
    println!();

    Ok(())
}
