use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database representation of a row in the `focus_sessions` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusSession {
    /// Primary key generated via UUID v4.
    pub id: Uuid,
    /// Owner of the session.
    pub user_id: Uuid,
    /// Optional task the session is linked to.
    pub task_id: Option<Uuid>,
    /// Planned focus duration expressed in seconds.
    pub duration_seconds: i32,
    /// Actual time spent in seconds.  Optional until the session finishes.
    pub actual_duration_seconds: Option<i32>,
    /// When the session started.
    pub started_at: DateTime<Utc>,
    /// When the session ended, if ever.
    pub ended_at: Option<DateTime<Utc>>,
    /// Whether the user indicated the session was completed.
    pub completed: bool,
    /// Free-form notes, stored for retrospection.
    pub notes: Option<String>,
    /// Insert timestamp for auditing.
    pub created_at: DateTime<Utc>,
}

/// Abstracts the information the focus-score endpoint needs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusScoreSample {
    pub duration_minutes: i32,
    pub interruptions: i32,
    pub state: FocusState,
}

/// User-facing session state for scoring heuristics.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FocusState {
    Active,
    Paused,
    Completed,
}

impl FocusScoreSample {
    pub fn focus_score(&self) -> f32 {
        if self.duration_minutes <= 0 {
            return 0.0;
        }
        let base = (self.duration_minutes as f32 / 60.0).min(1.0);
        let penalty = (self.interruptions as f32 * 0.1).min(0.5);
        let state_penalty = if self.state == FocusState::Completed {
            0.0
        } else {
            0.2
        };
        (base - penalty - state_penalty).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(overrides: Option<FocusScoreSample>) -> FocusScoreSample {
        overrides.unwrap_or(FocusScoreSample {
            duration_minutes: 25,
            interruptions: 0,
            state: FocusState::Completed,
        })
    }

    #[test]
    fn completed_session_has_positive_score() {
        let score = sample(None).focus_score();
        assert!(score > 0.0);
    }

    #[test]
    fn excessive_interruptions_clamp_to_zero() {
        let mut s = sample(None);
        s.interruptions = 20;
        assert_eq!(s.focus_score(), 0.0);
    }

    #[test]
    fn non_completed_state_penalized() {
        let mut paused = sample(None);
        paused.state = FocusState::Paused;
        assert!(paused.focus_score() < sample(None).focus_score());
    }

    #[test]
    fn non_positive_duration_returns_zero() {
        let mut s = sample(None);
        s.duration_minutes = 0;
        assert_eq!(s.focus_score(), 0.0);
    }
}
