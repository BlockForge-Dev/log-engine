# Data Model — log-engine v0.1

The input format is **NDJSON** (newline-delimited JSON).
Each line represents one independent log event.

---

## LogRow schema

All fields are optional to tolerate partial or inconsistent logs.

| Field      | Type           | Description                                        |
| ---------- | -------------- | -------------------------------------------------- |
| ts         | Option<i64>    | Timestamp (unix seconds or ms depending on source) |
| method     | Option<String> | RPC method or request name                         |
| chain      | Option<String> | Chain identifier (ethereum, solana, aptos, etc.)   |
| ip         | Option<String> | Client IP address                                  |
| status     | Option<u16>    | Status code (>= 400 counts as error)               |
| latency_ms | Option<f64>    | Latency in milliseconds                            |

---

## Interpretation rules

- `status >= 400` → increments `error_count`
- `latency_ms` contributes to:
  - total latency
  - max latency
- Missing fields do not cause parsing failure

---

## Example log lines

### Normal request

```json
{
  "ts": 1700000000,
  "method": "eth_call",
  "chain": "ethereum",
  "ip": "1.2.3.4",
  "status": 200,
  "latency_ms": 32.5
}
```

Error request
{"ts":1700000001,"method":"eth_sendRawTransaction","chain":"ethereum","ip":"1.2.3.4","status":500,"latency_ms":2100.0}

Missing fields (still valid)
{"ts":1700000002,"latency_ms":100.0}

Invalid JSON (counted as bad line)
{"ts":1700000003,"method":"eth_call"

Why this model

Real infrastructure logs are not clean

Optional fields prevent brittle pipelines

Supports gradual enrichment of log sources

---

# `docs/decisions.md`

```md
# Engineering Decisions — log-engine v0.1

This document records key design decisions and their tradeoffs.

---

## 1. NDJSON as input format

**Decision**

- Use newline-delimited JSON.

**Why**

- Natural fit for line-by-line processing.
- Easy to generate from most logging systems.
- Evolves naturally into streaming ingestion.

**Tradeoff**

- No global schema enforcement.
- Individual lines may be malformed.

---

## 2. Resilient parsing with Option fields

**Decision**

- All LogRow fields are optional.

**Why**

- Logs in production are inconsistent.
- Pipeline must continue even with partial data.

**Tradeoff**

- Missing fields reduce accuracy of aggregates.

---

## 3. HashMap-based aggregation

**Decision**

- Use `HashMap<String, Agg>` for per-group state.

**Why**

- Simple, correct baseline.
- Easy to reason about and refactor.

**Tradeoff**

- Memory usage grows with group cardinality.
- No eviction or bounding strategy yet.

---

## 4. Threshold-based anomaly detection

**Decision**

- Detect anomalies using fixed thresholds:
  - average latency
  - error rate
  - minimum sample size

**Why**

- Fully explainable.
- Good baseline for comparison with future statistical methods.

**Tradeoff**

- Averages hide tail latency.
- Static thresholds require tuning.

---

## 5. Batch/offline execution (Q1 scope)

**Decision**

- Keep v0.1 synchronous and offline.

**Why**

- Focus on Rust fundamentals and architecture clarity.
- Avoid premature async complexity.

**Tradeoff**

- Not suitable for real-time detection.

---

## 6. Human-readable output only

**Decision**

- Print text output, not JSON.

**Why**

- Optimized for developer feedback and debugging.

**Tradeoff**

- Harder to integrate into automated pipelines (planned in v0.2).

---

## Summary

v0.1 is intentionally conservative:

- correctness > performance
- clarity > features
- explainability > sophistication
```
