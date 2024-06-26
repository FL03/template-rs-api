/*
    Appellation: routes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod base;

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
        .route("/", get(base::home))
        .route("/sample/:id", get(base::get_sample))
        .route("/sample/:id", post(base::post_sample))
        .with_state(state)
}

// pub(crate) fn _api(ctx: AppState) -> ApiRouterTy {
//     Router::new().nest("/", v0()).layer(Extension(ctx))
// }
