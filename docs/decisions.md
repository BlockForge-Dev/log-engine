# Engineering Decisions — log-engine v0.1

This document records the **explicit design decisions** made in v0.1 of
`log-engine`, including _why_ each choice was made and _what tradeoffs_
were accepted.

The goal of v0.1 is **clarity, correctness, and architectural taste** —
not maximum performance or scale.

---

## 1. Use NDJSON as the input format

### Decision

Accept logs as **newline-delimited JSON (NDJSON)**, one event per line.

### Why

- NDJSON allows **line-by-line processing**, which maps cleanly to `BufRead`.
- It avoids loading the entire input into memory.
- It is a natural stepping stone toward **streaming ingestion** in later versions.
- Most logging systems can emit NDJSON easily.

### Tradeoffs

- No global schema enforcement.
- Individual lines may be malformed and must be handled defensively.

---

## 2. Parse logs into a permissive data model (`LogRow`)

### Decision

All fields in `LogRow` are optional (`Option<T>`), with `#[serde(default)]`.

### Why

- Real-world logs are often incomplete or inconsistent.
- The pipeline should not crash because a field is missing.
- This favors **robustness and availability** over strict validation.

### Tradeoffs

- Missing fields reduce analytical precision.
- Some anomalies may be underreported (e.g., missing status codes).

---

## 3. Count and skip bad lines instead of failing fast

### Decision

If a line cannot be read or parsed as JSON:

- increment `bad_lines`
- continue processing the rest of the input

### Why

- In operational systems, partial data is better than no data.
- One corrupted line should not invalidate the entire analysis.
- Keeps the pipeline resilient under imperfect input conditions.

### Tradeoffs

- Silent data loss for malformed lines.
- Requires users to inspect `bad_lines` to assess input quality.

---

## 4. Group logs using a simple string-based strategy

### Decision

Group logs by a string key derived from:

- `method`
- `chain`
- `ip`

Any unknown or missing value falls back to `"unknown_*"`.

### Why

- Keeps grouping logic simple and explicit.
- Avoids introducing enums or traits prematurely in v0.1.
- Makes the grouping strategy easy to reason about and change.

### Tradeoffs

- String-based grouping is error-prone (typos).
- `"unknown_*"` values may merge unrelated events.

_(Planned improvement: typed `GroupBy` enum in v0.2.)_

---

## 5. Use `HashMap<String, Agg>` for aggregation

### Decision

Store per-group aggregation state in a `HashMap` keyed by group string.

### Why

- HashMap provides O(1) average insert/update time.
- Simple and correct baseline for grouping problems.
- Keeps aggregation logic localized and testable.

### Tradeoffs

- Memory grows with the number of unique groups.
- No eviction, bounding, or top-k strategy yet.

---

## 6. Track simple aggregation metrics only

### Decision

Each group tracks:

- total count
- total latency
- max latency
- error count

Derived metrics:

- average latency
- error rate

### Why

- These metrics are easy to explain and validate.
- They are sufficient for a first anomaly signal.
- Keeps mathematical complexity low in v0.1.

### Tradeoffs

- Averages hide tail latency (p95/p99).
- No variance or distribution awareness.

---

## 7. Use threshold-based anomaly detection

### Decision

Flag a group as anomalous when:

- `avg_latency > latency_threshold`, OR
- `error_rate > error_rate_threshold`,
- AND `count >= min_samples`.

### Why

- Threshold rules are **fully explainable**.
- Easy to reason about during early system development.
- Provides a baseline to compare against statistical methods later.

### Tradeoffs

- Static thresholds require tuning.
- Cannot adapt to changing baselines.
- Sensitive to workload mix changes.

_(Planned improvement: EWMA-based baselines in v1.0.)_

---

## 8. Batch, offline execution (no async, no streaming)

### Decision

v0.1 runs as a **single-process, synchronous, batch** CLI.

### Why

- Q1 focus is Rust fundamentals and system structure.
- Avoids premature complexity (async runtimes, backpressure).
- Makes behavior deterministic and debuggable.

### Tradeoffs

- Not suitable for real-time anomaly detection.
- Cannot process unbounded streams.

---

## 9. Human-readable output only

### Decision

Output results as formatted text to stdout.

### Why

- Optimized for developer feedback.
- Makes anomalies immediately visible.
- Simplifies early iteration and debugging.

### Tradeoffs

- Not machine-friendly.
- Harder to integrate into automated pipelines.

_(Planned improvement: optional JSON output in v0.2.)_

---

## 10. Favor architectural clarity over extensibility

### Decision

Split the system into small, explicit modules:

- `cli`
- `model`
- `group`
- `agg`
- `run`

### Why

- Makes responsibilities obvious.
- Encourages reasoning about boundaries.
- Supports incremental evolution without rewrites.

### Tradeoffs

- Slightly more files than a single-file script.
- Some abstractions are intentionally minimal.

---

## Summary

`log-engine` v0.1 is intentionally conservative:

- **Correctness over performance**
- **Clarity over features**
- **Explainability over sophistication**

Each decision creates a clean foundation for:

- streaming ingestion
- adaptive anomaly detection
- bounded-memory aggregation
- production hardening in later versions
