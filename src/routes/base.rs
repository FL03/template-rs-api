/*
    Appellation: base <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::AppState;
use axum::extract::{Json, Path, State};
use serde_json::Value;

pub async fn home(State(ctx): State<AppState>) -> Json<Value> {
    let samples = ctx.fetch_samples().await.unwrap_or_default();
    let data = serde_json::json!({
        "message": "Hello, World!",
        "mode": ctx.settings().mode(),
        "samples": samples,
    });
    axum::Json(data)
}

pub async fn get_sample(id: Path<String>) -> Json<Value> {
    let data = serde_json::json!({
        "id": id.as_str(),
    });
    axum::Json(data)
}

pub async fn post_sample(id: Path<String>) -> Json<Value> {
    let data = serde_json::json!({
        "id": id.as_str(),
    });
    axum::Json(data)
}
