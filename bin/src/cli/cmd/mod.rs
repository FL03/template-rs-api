/*
   Appellation: cmd <cli>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::system::System;

mod system;

use clap::Subcommand;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Clone, Debug, Deserialize, Display, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Subcommand)]
pub enum Commands {
    System(System),
}

impl Default for Commands {
    fn default() -> Self {
        Self::System(Default::default())
    }
}
