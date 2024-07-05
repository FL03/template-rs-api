/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod crud;

use std::sync::{Arc, Mutex};

pub type Arcm<T = ()> = Arc<Mutex<T>>;

pub type AppState = Arc<crate::ctx::Context>;
/// A type alias for `axum`'s [Router](axum::Router)
pub type ApiRouter<S = ()> = axum::Router<S>;

pub(crate) mod prelude {
    pub use super::crud::CRUD;
    pub use super::{ApiRouter, AppState, Arcm};
}
