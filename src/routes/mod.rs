/*
    Appellation: routes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod items;

use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub(crate) fn v0<S>(state: AppState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .nest("/items", items_router())
        .with_state(state)
}

fn items_router() -> Router<AppState> {
    Router::new()
        .route("/", get(items::get_items))
        .route("/:id", get(items::get_item))
        .route("/:id", post(items::post_item))
}
