/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused_imports)]
pub use self::prelude::*;

pub(crate) mod prelude {
    pub use crate::primitives::consts::*;
    pub use crate::primitives::types::*;
}

pub mod consts {
    pub const APP_NAME: &str = "tapp"; // env!("CARGO_PKG_NAME");
    pub const ENV_PREFIX: &str = "APP";
}

pub mod types {
    use std::sync::{Arc, Mutex};

    pub type Arcm<T = ()> = Arc<Mutex<T>>;

    pub type AppState = Arc<crate::ctx::Context>;
    /// A type alias for `axum`'s [Router](axum::Router)
    pub type ApiRouter<S = ()> = axum::Router<S>;
}
