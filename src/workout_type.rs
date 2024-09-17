//! Used as the `type` field for a workout set.

use serde::{Deserialize, Serialize};

/// The type of set for a specific workout.
///
/// Used as the `type` field for a workout set.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SetType {
    /// A warmup set.
    Warmup,
    /// A normal working set.
    Normal,
    /// A failed set, or set to failure.
    Failure,
    /// A drop set.
    Drop,
}
