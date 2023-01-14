/*
    Appellation: build <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::command;
use anyhow::Result;
use clap::{Args, ArgAction};

pub fn builder(release: bool, workspace: bool) -> Result<()> {
    let mut args = vec!["build"];

    if release {
        args.push("--release");
    }
    if workspace {
        args.push("--workspace");
    }
    command("cargo", args)?;
    Ok(())
}

#[derive(Args, Debug, Default, Eq, Hash, PartialEq)]
pub struct Build {
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub release: bool,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub workspace: bool
}

impl Build {
    pub fn new(release: bool, workspace: bool) -> Self {
        Self { release, workspace }
    }
    pub fn process(&self) -> Result<&Self> {
        builder(self.release, self.workspace)?;
        Ok(self)
    }
}