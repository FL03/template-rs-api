/*
   Appellation: system <cli>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use clap::{ArgAction, Args};
use serde::{Deserialize, Serialize};

#[derive(
    Args, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct System {
    #[clap(long, short)]
    pub config: Option<std::path::PathBuf>,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub detached: bool,
    #[arg(action = ArgAction::SetTrue, long, short = 'U')]
    pub up: bool,
}

impl System {
    pub async fn handle(&self) -> anyhow::Result<&Self> {
        Ok(self)
    }
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
