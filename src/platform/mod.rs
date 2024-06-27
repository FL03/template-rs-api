/*
    Appellation: platform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{init::*, platform::*, state::PlatformState};

pub(crate) mod init;
pub(crate) mod platform;
pub(crate) mod state;
