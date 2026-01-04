# Architecture — log-engine v0.1

This document explains the architecture of `log-engine` v0.1 using a
systems-thinking approach.

The goal of v0.1 is **clarity and correctness**, not scale or streaming.

---

## High-level overview

`log-engine` is a single-process, batch-oriented log analysis pipeline:

NDJSON (stdin/file)
|
v
Line Reader
|
v
JSON Parser (LogRow)
|
v
Grouping Strategy (method | chain | ip)
|
v
Aggregation (per-group state)
|
v
Anomaly Detection + Reporting

---

## Components and why they exist

### 1. CLI Layer (`cli.rs`)

**Responsibility**

- Defines the user-facing contract (flags, defaults, help text).

**Why it exists**

- Separates configuration from execution logic.
- Makes the system easier to evolve (new flags don’t touch core logic).

---

### 2. Ingestion Layer (`run.rs`)

**Responsibility**

- Reads input from file or stdin line-by-line using `BufRead`.

**Why it exists**

- NDJSON is naturally processed as a stream of lines.
- Avoids loading entire files into memory.
- Enables future streaming refactor with minimal changes.

---

### 3. Parsing Layer (`model.rs`)

**Responsibility**

- Deserialize each line into `LogRow`.

**Why it exists**

- Real logs are messy.
- All fields are optional to prevent crashes from partial data.
- Bad lines are counted and skipped instead of aborting execution.

---

### 4. Grouping Strategy (`group.rs`)

**Responsibility**

- Maps a `LogRow` to a group key (`method`, `chain`, or `ip`).

**Why it exists**

- Grouping is a policy decision, not aggregation logic.
- This is a clean seam for adding new grouping strategies later.

---

### 5. Aggregation State (`agg.rs`)

**Responsibility**

- Maintains per-group counters:
  - count
  - latency_sum
  - latency_max
  - error_count

**Why it exists**

- Encapsulates all numeric logic.
- Keeps math isolated from I/O and parsing concerns.
- Makes correctness review straightforward.

---

### 6. Detection + Reporting (`run.rs`)

**Responsibility**

- Computes derived metrics:
  - average latency
  - error rate
- Flags anomalies using thresholds.
- Prints human-readable output.

**Why it exists**

- v0.1 prioritizes explainability over statistical complexity.
- Output is designed for fast human feedback.

---

## Scale assumptions (v0.1)

This version is designed for:

- Single-machine execution
- Offline analysis
- Moderate log sizes

### Complexity

- **CPU**: O(N) where N = number of log lines
- **Memory**: O(G) where G = number of unique groups

### What does not scale yet

- Very high cardinality grouping (e.g. millions of IPs)
- Real-time or continuous ingestion

---

## Failure modes

### Input failures

- File read errors → counted as `bad_lines`
- stdin read errors → counted as `bad_lines`

### Data failures

- Invalid JSON → counted as `bad_lines`
- Missing fields → replaced with `"unknown_*"` or ignored
- Missing latency → affects averages but not counts
- Missing status → error rate may be underestimated

### Known edge case (handled)

- NaN latency values could break sorting
- v0.1 includes NaN-safe sorting logic

---

## Data consistency tradeoffs

- Batch execution → deterministic results for the same input
- Bad lines are dropped → favors availability of results
- Default `"unknown_*"` grouping → favors robustness over accuracy

This system does **best-effort analysis**, not strict accounting.

---

## Why this design (v0.1)

- Simple pipeline keeps the mental model clear
- Clean boundaries allow incremental evolution:
  - v1.0: streaming ingestion
  - v2.0: bounded memory + load shedding
- Designed to teach systems thinking, not just produce output
