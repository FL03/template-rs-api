/*
    Appellation: base <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::AppState;
use crate::models::{ItemId, ItemModel};
use axum::extract::{Json, Path, State};
use serde_json::Value;

pub async fn get_items(State(ctx): State<AppState>) -> Json<Vec<ItemModel>> {
    let items = ctx.get_items().await.unwrap_or_default();
    axum::Json(items)
}

pub async fn get_item(State(ctx): State<AppState>, id: Path<ItemId>) -> Json<Option<ItemModel>> {
    let item = ctx.get_item(id.0).await.ok();
    
    axum::Json(item)
}

pub async fn post_item(State(ctx): State<AppState>, title: Path<String>) -> Json<Value> {
    let item = ctx.add_item(title.0, String::new()).await;
    let data = match item {
        Ok(item) => {
            serde_json::json!({
                "result": "success",
                "data": item,
            })
        }
        Err(e) => {
            serde_json::json!({
                "result": "error",
                "message": e.to_string(),
            })
        }
    };
    axum::Json(data)
}


