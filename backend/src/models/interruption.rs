use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use uuid::Uuid;

/// Enumerates why a focus session was interrupted.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InterruptionCategory {
    External,
    Internal,
    Urgent,
    Break,
}

impl fmt::Display for InterruptionCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterruptionCategory::External => write!(f, "external"),
            InterruptionCategory::Internal => write!(f, "internal"),
            InterruptionCategory::Urgent => write!(f, "urgent"),
            InterruptionCategory::Break => write!(f, "break"),
        }
    }
}

impl FromStr for InterruptionCategory {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "external" => Ok(Self::External),
            "internal" => Ok(Self::Internal),
            "urgent" => Ok(Self::Urgent),
            "break" => Ok(Self::Break),
            _ => Err("invalid interruption category"),
        }
    }
}

/// Row stored in the `interruptions` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interruption {
    pub id: Uuid,
    pub session_id: Uuid,
    pub category: InterruptionCategory,
    pub reason: Option<String>,
    pub duration_seconds: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interruption_category_round_trip() {
        for (raw, variant) in [
            ("external", InterruptionCategory::External),
            ("internal", InterruptionCategory::Internal),
            ("urgent", InterruptionCategory::Urgent),
            ("break", InterruptionCategory::Break),
        ] {
            assert_eq!(InterruptionCategory::from_str(raw).unwrap(), variant);
            assert_eq!(variant.to_string(), raw);
        }
    }
}
