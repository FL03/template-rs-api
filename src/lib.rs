/*
    Appellation: template-rs-api <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{
    config::Settings, ctx::Context, platform::Platform, primitives::*, server::Server,
    types::prelude::*,
};

pub(crate) mod primitives;

pub mod api;
pub mod config;
pub mod ctx;
pub mod models;
pub mod platform;
pub mod routes;
pub mod server;
pub mod types;

pub mod prelude {
    pub use crate::config::Settings;
    pub use crate::consts::*;
    pub use crate::platform::Platform;
    pub use crate::server::Server;
    pub use crate::types::prelude::*;
}
