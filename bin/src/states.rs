/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::prelude::Hashable;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Deserialize, Display, EnumString, EnumVariantNames, Eq, Hash, Hashable, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum State {
    Error = 0,
    Idle = 1,
    Complete = 2,
    Derive = 3,
    Process = 4,
    Request = 5,
    Response = 6,
}

impl State {
    pub fn idle() -> Self {
        Self::Idle
    }
}

impl Default for State {
    fn default() -> Self {
        Self::idle()
    }
}

impl From<State> for i64 {
    fn from(val: State) -> Self {
        val as i64
    }
}
