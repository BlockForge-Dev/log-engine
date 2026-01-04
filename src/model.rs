use serde::Deserialize;

/// One NDJSON row. All fields are optional to be resilient to messy real logs.
#[derive(Debug, Deserialize)]
pub struct LogRow {
    #[allow(dead_code)]
    #[serde(default)]
    pub ts: Option<i64>,

    #[serde(default)]
    pub method: Option<String>,

    #[serde(default)]
    pub chain: Option<String>,

    #[serde(default)]
    pub ip: Option<String>,

    #[serde(default)]
    pub status: Option<u16>, // e.g. 200, 401, 500

    #[serde(default)]
    pub latency_ms: Option<f64>,
}
