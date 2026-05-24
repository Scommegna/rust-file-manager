use std::fs;
use std::time::{Duration, Instant};

pub struct Benchmark {
    start_time: Instant,
}

pub struct BenchmarkResult {
    pub elapsed_time: Duration,
    pub memory_usage_kb: Option<u64>,
}

impl Benchmark {
    pub fn start_benchmark() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    pub fn stop_benchmark(&self) -> BenchmarkResult {
        BenchmarkResult {
            elapsed_time: self.start_time.elapsed(),
            memory_usage_kb: get_memory_usage_kb(),
        }
    }
}

pub fn get_memory_usage_kb() -> Option<u64> {
    let status = fs::read_to_string("/proc/self/status").ok()?;

    for line in status.lines() {
        if let Some(value) = line.strip_prefix("VmRSS:") {
            let mut parts = value.split_whitespace();
            return parts.next()?.parse::<u64>().ok();
        }
    }

    None
}

pub fn print_benchmark(result: &BenchmarkResult) {
    println!("\n[Benchmark]");
    println!("Execution time: {} ms", result.elapsed_time.as_millis());

    match result.memory_usage_kb {
        Some(memory_usage_kb) => println!("Resident memory: {} kb", memory_usage_kb),
        None => println!("Resident memory: not available"),
    }
}
