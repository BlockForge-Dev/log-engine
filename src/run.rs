use crate::{agg::Agg, cli::Args, group::group_key, model::LogRow};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Run the log engine pipeline:
/// - build reader (file/stdin)
/// - parse NDJSON lines
/// - aggregate by group key
/// - print summary + anomalies + top groups
pub fn run(args: Args) -> anyhow::Result<()> {
    let reader: Box<dyn BufRead> = match args.input.as_deref() {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut aggs: HashMap<String, Agg> = HashMap::new();
    let mut parsed = 0u64;
    let mut bad_lines = 0u64;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                bad_lines += 1;
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<LogRow>(&line) {
            Ok(row) => {
                parsed += 1;
                let key = group_key(&row, args.group_by);

                aggs.entry(key).or_default().observe(&row);
            }
            Err(_) => {
                bad_lines += 1;
            }
        }
    }

    // Print summary
    println!(
        "parsed_lines={} bad_lines={} groups={}",
        parsed,
        bad_lines,
        aggs.len()
    );

    // Find anomalies
    println!("\nANOMALIES:");
    for (k, a) in aggs.iter() {
        if a.count < args.min_samples {
            continue;
        }

        let avg_lat = a.avg_latency();
        let err_rate = a.error_rate();

        let latency_bad = avg_lat > args.latency_threshold;
        let error_bad = err_rate > args.error_rate_threshold;

        if latency_bad || error_bad {
            println!(
                "- key={} samples={} avg_latency_ms={:.2} max_latency_ms={:.2} error_rate={:.2} (errors={})",
                k, a.count, avg_lat, a.latency_max, err_rate, a.error_count
            );
        }
    }

    // Top groups (by avg latency)
    println!("\nTOP GROUPS (by avg latency):");

    let mut rows: Vec<_> = aggs.iter().collect();

    // NaN-safe sort: treat NaN as lowest priority (push to end).
    rows.sort_by(|a, b| {
        let a_avg = a.1.avg_latency();
        let b_avg = b.1.avg_latency();

        // If either side is NaN, push it "down".
        match (a_avg.is_nan(), b_avg.is_nan()) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Greater, // a after b
            (false, true) => std::cmp::Ordering::Less,    // a before b
            (false, false) => b_avg
                .partial_cmp(&a_avg)
                .unwrap_or(std::cmp::Ordering::Equal),
        }
    });

    for (k, a) in rows.into_iter().take(10) {
        println!(
            "- key={} samples={} avg_latency_ms={:.2} max_latency_ms={:.2} error_rate={:.2}",
            k,
            a.count,
            a.avg_latency(),
            a.latency_max,
            a.error_rate()
        );
    }

    Ok(())
}
