use clap::{Parser, ValueEnum};

/// Controls which field we group logs by.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GroupBy {
    Method,
    Chain,
    Ip,
}

/// log-engine v0.1 - offline NDJSON analyzer (group + aggregate + threshold anomalies)
#[derive(Parser, Debug)]
#[command(name = "log-engine-v0.1")]
pub struct Args {
    /// Path to NDJSON log file. If omitted, reads from stdin.
    #[arg(long)]
    pub input: Option<String>,

    /// Group key for aggregation
    #[arg(long, value_enum, default_value_t = GroupBy::Method)]
    pub group_by: GroupBy,

    /// Flag anomaly when avg latency for a group exceeds this
    #[arg(long, default_value_t = 1500.0)]
    pub latency_threshold: f64,

    /// Flag anomaly when error rate for a group exceeds this (requires `status`)
    #[arg(long, default_value_t = 0.10)]
    pub error_rate_threshold: f64,

    /// Minimum samples per group before we judge anomalies
    #[arg(long, default_value_t = 20)]
    pub min_samples: u64,
}
