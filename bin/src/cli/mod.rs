/*
   Appellation: cli <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub mod opts;

use clap::{ArgAction, Parser};
use opts::Opts;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct CommandLineInterface {
    #[clap(subcommand)]
    pub cmd: Option<Opts>,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub verbose: bool,
}

impl CommandLineInterface {
    pub fn new() -> Self {
        Self::parse()
    }
    pub fn command(&self) -> Option<Opts> {
        self.cmd.clone()
    }
    pub fn verbose(&self) -> bool {
        self.verbose
    }
}

impl Default for CommandLineInterface {
    fn default() -> Self {
        Self::parse()
    }
}

impl std::fmt::Display for CommandLineInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
