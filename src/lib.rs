/*
    Appellation: template-rs-api <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{app::App, primitives::prelude::*, server::Server};
pub use self::config::{Context, Settings};

pub(crate) mod primitives;

pub mod app;
pub mod config;
pub mod routes;
pub mod server;

pub mod prelude {
    pub use crate::app::App;
    pub use crate::config::Settings;
    pub use crate::primitives::types::*;
    pub use crate::server::Server;
}