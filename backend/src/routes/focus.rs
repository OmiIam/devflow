use axum::{http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::models::focus_session::{FocusScoreSample, FocusState};

#[derive(Debug, Deserialize)]
pub struct FocusScoreRequest {
    pub duration_minutes: i32,
    pub interruptions: i32,
    pub state: FocusState,
}

#[derive(Debug, Serialize)]
pub struct FocusScoreResponse {
    pub score: f32,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn calculate_focus_score(
    Json(payload): Json<FocusScoreRequest>,
) -> Result<Json<FocusScoreResponse>, (StatusCode, Json<ErrorResponse>)> {
    if payload.duration_minutes <= 0 {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ErrorResponse {
                error: "duration_minutes must be positive".into(),
            }),
        ));
    }

    let session = FocusScoreSample {
        duration_minutes: payload.duration_minutes,
        interruptions: payload.interruptions,
        state: payload.state,
    };

    Ok(Json(FocusScoreResponse {
        score: session.focus_score(),
    }))
}

pub fn router() -> Router {
    Router::new().route("/focus/score", post(calculate_focus_score))
}
