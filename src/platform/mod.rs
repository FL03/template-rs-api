/*
    Appellation: platform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{init::*, platform::*, server::Server, state::*};

pub(crate) mod init;
pub(crate) mod platform;
pub(crate) mod state;

pub mod server;

pub(crate) mod prelude {
    pub use super::init::Initializer;
    pub use super::platform::Platform;
    pub use super::server::Server;
    pub use super::state::PlatformState;
}
