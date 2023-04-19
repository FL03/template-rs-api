/*
   Appellation: interface <cli>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::cli::cmd::Commands;
use clap::{ArgAction, Parser};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct CommandLineInterface {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[clap(long, short, default_value_t = SocketAddr::from(([0, 0, 0, 0], 8080)))]
    pub addr: Option<SocketAddr>,
    #[arg(action = ArgAction::SetTrue, default_value_t = false, long, short)]
    pub debug: bool,
}

impl CommandLineInterface {
    pub fn command(&self) -> Option<Commands> {
        self.command.clone()
    }
    pub fn debug(&self) -> bool {
        self.debug
    }
}

impl Default for CommandLineInterface {
    fn default() -> Self {
        Self::parse()
    }
}
