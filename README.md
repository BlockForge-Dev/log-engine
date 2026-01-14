
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
