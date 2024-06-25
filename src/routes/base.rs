/*
    Appellation: base <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use axum::{
    extract::{Json, Path},
    Extension,
};
use serde_json::Value;

pub async fn home() -> Json<Value> {
    let data = serde_json::json!({
        "message": "Hello, World!",
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
