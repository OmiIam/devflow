use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FocusState {
    Active,
    Paused,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub duration_minutes: i32,
    pub interruptions: i32,
    pub state: FocusState,
}

impl FocusSession {
    pub fn focus_score(&self) -> f32 {
        if self.duration_minutes <= 0 {
            return 0.0;
        }
        let base = (self.duration_minutes as f32 / 60.0).min(1.0);
        let penalty = (self.interruptions as f32 * 0.1).min(0.5);
        let state_penalty = if self.state == FocusState::Completed { 0.0 } else { 0.2 };
        (base - penalty - state_penalty).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session(overrides: Option<FocusSession>) -> FocusSession {
        overrides.unwrap_or(FocusSession {
            id: Uuid::nil(),
            user_id: Uuid::nil(),
            duration_minutes: 25,
            interruptions: 0,
            state: FocusState::Completed,
        })
    }

    #[test]
    fn completed_session_has_positive_score() {
        let score = session(None).focus_score();
        assert!(score > 0.0);
    }

    #[test]
    fn excessive_interruptions_clamp_to_zero() {
        let mut s = session(None);
        s.interruptions = 20;
        assert_eq!(s.focus_score(), 0.0);
    }

    #[test]
    fn non_completed_state_penalized() {
        let mut paused = session(None);
        paused.state = FocusState::Paused;
        assert!(paused.focus_score() < session(None).focus_score());
    }

    #[test]
    fn non_positive_duration_returns_zero() {
        let mut s = session(None);
        s.duration_minutes = 0;
        assert_eq!(s.focus_score(), 0.0);
    }
}
