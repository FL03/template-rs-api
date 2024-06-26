/*
    Appellation: template-rs-api <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{app::App, config::Settings, ctx::Context, primitives::prelude::*, server::Server};

pub(crate) mod primitives;

pub mod app;
pub mod config;
pub mod ctx;
pub mod routes;
pub mod server;
pub mod states;

pub mod prelude {
    pub use crate::app::App;
    pub use crate::config::Settings;
    pub use crate::primitives::types::*;
    pub use crate::server::Server;
}
