// use clap::Parser;
// use serde::Deserialize;
// use std::collections::HashMap;
// use std::fs::File;
// use std::io::{self, BufRead, BufReader};

// #[derive(Parser, Debug)]
// #[command(name = "log-engine-v0.1")]
// struct Args {
//     /// Path to NDJSON log file. If omitted, reads from stdin.
//     #[arg(long)]
//     input: Option<String>,

//     /// Group key: method | chain | ip (based on fields present)
//     #[arg(long, default_value = "method")]
//     group_by: String,

//     /// Flag anomaly when avg latency for a group exceeds this
//     #[arg(long, default_value_t = 1500.0)]
//     latency_threshold: f64,

//     /// Flag anomaly when error rate for a group exceeds this (requires `status`)
//     #[arg(long, default_value_t = 0.10)]
//     error_rate_threshold: f64,

//     /// Minimum samples per group before we judge anomalies
//     #[arg(long, default_value_t = 20)]
//     min_samples: u64,
// }

// #[derive(Debug, Deserialize)]
// struct LogRow {
//     #[serde(default)]
//     ts: Option<i64>,

//     #[serde(default)]
//     method: Option<String>,

//     #[serde(default)]
//     chain: Option<String>,

//     #[serde(default)]
//     ip: Option<String>,

//     #[serde(default)]
//     status: Option<u16>, // e.g. 200, 401, 500

//     #[serde(default)]
//     latency_ms: Option<f64>,
// }

// #[derive(Debug, Default, Clone)]
// struct Agg {
//     count: u64,
//     latency_sum: f64,
//     latency_max: f64,
//     error_count: u64,
// }

// impl Agg {
//     fn observe(&mut self, row: &LogRow) {
//         self.count += 1;

//         if let Some(lat) = row.latency_ms {
//             self.latency_sum += lat;
//             if lat > self.latency_max {
//                 self.latency_max = lat;
//             }
//         }

//         if let Some(status) = row.status {
//             if status >= 400 {
//                 self.error_count += 1;
//             }
//         }
//     }

//     fn avg_latency(&self) -> f64 {
//         if self.count == 0 {
//             0.0
//         } else {
//             self.latency_sum / (self.count as f64)
//         }
//     }

//     fn error_rate(&self) -> f64 {
//         if self.count == 0 {
//             0.0
//         } else {
//             self.error_count as f64 / self.count as f64
//         }
//     }
// }

// fn group_key(row: &LogRow, group_by: &str) -> String {
//     match group_by {
//         "chain" => row.chain.clone().unwrap_or_else(|| "unknown_chain".into()),
//         "ip" => row.ip.clone().unwrap_or_else(|| "unknown_ip".into()),
//         _ => row
//             .method
//             .clone()
//             .unwrap_or_else(|| "unknown_method".into()),
//     }
// }

// fn main() -> anyhow::Result<()> {
//     let args = Args::parse();

//     let reader: Box<dyn BufRead> = match args.input.as_deref() {
//         Some(path) => Box::new(BufReader::new(File::open(path)?)),
//         None => Box::new(BufReader::new(io::stdin())),
//     };

//     let mut aggs: HashMap<String, Agg> = HashMap::new();
//     let mut parsed = 0u64;
//     let mut bad_lines = 0u64;

//     for line in reader.lines() {
//         let line = match line {
//             Ok(l) => l,
//             Err(_) => {
//                 bad_lines += 1;
//                 continue;
//             }
//         };

//         if line.trim().is_empty() {
//             continue;
//         }

//         match serde_json::from_str::<LogRow>(&line) {
//             Ok(row) => {
//                 parsed += 1;
//                 let key = group_key(&row, &args.group_by);
//                 aggs.entry(key).or_default().observe(&row);
//             }
//             Err(_) => {
//                 bad_lines += 1;
//             }
//         }
//     }

//     // Print summary
//     println!(
//         "parsed_lines={} bad_lines={} groups={}",
//         parsed,
//         bad_lines,
//         aggs.len()
//     );

//     // Find anomalies
//     println!("\nANOMALIES:");
//     for (k, a) in aggs.iter() {
//         if a.count < args.min_samples {
//             continue;
//         }

//         let avg_lat = a.avg_latency();
//         let err_rate = a.error_rate();

//         let latency_bad = avg_lat > args.latency_threshold;
//         let error_bad = err_rate > args.error_rate_threshold;

//         if latency_bad || error_bad {
//             println!(
//                 "- key={} samples={} avg_latency_ms={:.2} max_latency_ms={:.2} error_rate={:.2} (errors={})",
//                 k, a.count, avg_lat, a.latency_max, err_rate, a.error_count
//             );
//         }
//     }

//     println!("\nTOP GROUPS (by avg latency):");
//     let mut rows: Vec<_> = aggs.iter().collect();
//     rows.sort_by(|a, b| b.1.avg_latency().partial_cmp(&a.1.avg_latency()).unwrap());
//     for (k, a) in rows.into_iter().take(10) {
//         println!(
//             "- key={} samples={} avg_latency_ms={:.2} max_latency_ms={:.2} error_rate={:.2}",
//             k,
//             a.count,
//             a.avg_latency(),
//             a.latency_max,
//             a.error_rate()
//         );
//     }

//     Ok(())
// }

mod agg;
mod cli;
mod group;
mod model;
mod run;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    run::run(args)
}
