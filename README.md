This log engine processes NDJSON logs in a streaming, line-by-line fashion and aggregates metrics by a selected key (default: method). It is designed for local processing of large log files (hundreds of MB to ~1GB) and supports thousands of groups efficiently. Very high-cardinality keys (e.g., grouping by IP with hundreds of thousands/millions of unique values) may require additional controls (top-K/eviction), planned for later versions.




        ┌───────────────┐
        │   Input Src   │
        │ file / stdin  │
        └───────┬───────┘
                │ bytes
                v
        ┌───────────────┐
        │  Reader/I/O   │   (BufRead lines)
        └───────┬───────┘
                │ line (String/Bytes)
                v
        ┌───────────────┐
        │ Parser/Decoder│   (NDJSON → LogEvent)
        └───────┬───────┘
                │ LogEvent | ParseError
                v
        ┌───────────────┐
        │ Normalizer     │  (derive keys/fields, defaults)
        └───────┬───────┘
                │ NormalizedEvent
                v
        ┌───────────────┐
        │ Aggregator     │  (HashMap<Key, AggState>)
        │ + Windows      │  (optional in Q1: all-time / simple window)
        └───────┬───────┘
                │ Snapshot / Metrics
                v
        ┌───────────────┐
        │ Detector       │  (thresholds/rules)
        └───────┬───────┘
                │ AnomalyEvent(s)
                v
   ┌────────────┴─────────────┐
   │           Output           │
   │ CLI summary / JSON / sink  │
   └────────────────────────────┘


# log-engine (v0.1.0)

Offline NDJSON log analyzer in Rust.

It reads logs from a file or stdin, groups by `method | chain | ip`, aggregates latency + error rate,
and prints anomalies and top groups.

## Quickstart

```bash
cargo run -- --input examples/sample.ndjson --group-by method --min-samples 1
Input format (NDJSON)
One JSON object per line. Supported fields:

ts, method, chain, ip, status, latency_ms

Example:

json
Copy code
{"ts":1700000000,"method":"eth_call","chain":"ethereum","ip":"1.2.3.4","status":200,"latency_ms":32.5}
Docs
docs/architecture.md

docs/data_model.md

docs/decisions.md
```



This CLI processes NDJSON logs in a streaming fashion (line-by-line) and aggregates metrics per group key.
It is designed for local files up to ~1GB and up to tens of thousands of unique groups.
Extremely high-cardinality keys (e.g., grouping by IP with millions of unique IPs) may cause high memory usage.
