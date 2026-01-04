use crate::model::LogRow;

/// Per-group aggregation state.
#[derive(Debug, Default, Clone)]
pub struct Agg {
    pub count: u64,
    pub latency_sum: f64,
    pub latency_max: f64,
    pub error_count: u64,
}

impl Agg {
    /// Update aggregation state with one log row.
    pub fn observe(&mut self, row: &LogRow) {
        self.count += 1;

        if let Some(lat) = row.latency_ms {
            self.latency_sum += lat;
            if lat > self.latency_max {
                self.latency_max = lat;
            }
        }

        if let Some(status) = row.status {
            if status >= 400 {
                self.error_count += 1;
            }
        }
    }

    /// Average latency for this group (ms).
    pub fn avg_latency(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.latency_sum / (self.count as f64)
        }
    }

    /// Error rate (0..1) for this group.
    pub fn error_rate(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.error_count as f64 / self.count as f64
        }
    }
}
