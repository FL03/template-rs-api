/*
    Appellation: routes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod base;

use crate::config::Context;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub(crate) fn v0<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/", get(base::home))
        .route("/sample/:id", get(base::get_sample))
        .route("/sample/:id", post(base::post_sample))
}

pub(crate) fn _api(ctx: Arc<Context>) -> Router {
    Router::new()
        .nest("/", v0())
        .layer(axum::Extension(ctx))
}
