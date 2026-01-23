use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::state::AppState;

pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "status": "ok",
        "database": "connected"
    }))
}
