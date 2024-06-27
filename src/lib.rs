/*
    Appellation: template-rs-api <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{
    config::Settings, ctx::Context, platform::Platform, primitives::prelude::*, server::Server,
};

pub(crate) mod primitives;

pub mod config;
pub mod ctx;
pub mod models;
pub mod platform;
pub mod routes;
pub mod server;

pub mod prelude {
    pub use crate::config::Settings;
    pub use crate::platform::Platform;
    pub use crate::primitives::types::*;
    pub use crate::server::Server;
}
