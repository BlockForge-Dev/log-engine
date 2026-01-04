use crate::model::LogRow;

/// Determine grouping key for a row based on `group_by`.
///
/// Supported values:
/// - "method" (default)
/// - "chain"
/// - "ip"
///
/// Any other value falls back to "method" behavior.
use crate::cli::GroupBy;

/// Determine grouping key for a row based on `GroupBy`.
pub fn group_key(row: &LogRow, group_by: GroupBy) -> String {
    match group_by {
        GroupBy::Chain => row.chain.clone().unwrap_or_else(|| "unknown_chain".into()),
        GroupBy::Ip => row.ip.clone().unwrap_or_else(|| "unknown_ip".into()),
        GroupBy::Method => row
            .method
            .clone()
            .unwrap_or_else(|| "unknown_method".into()),
    }
}
