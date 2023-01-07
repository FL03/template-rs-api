/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::args::{AccountArgs, System};
use clap::Subcommand;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, Subcommand)]
pub enum Commands {
    Account(AccountArgs),
    System(System),
}

impl Commands {
    pub async fn handler(&self, ctx: Arc<crate::Context>) -> AsyncResult {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Account(acct) => {
                acct.handler().await?;
            },
            Self::System(system) => {
                system.handler(ctx).await?;
            }
        };
        Ok(())
    }
}

pub async fn handle(cmd: Commands, ctx: Arc<crate::Context>) -> AsyncResult {
    tracing::info!("Processing commands issued to the cli...");
    match cmd {
        Commands::Account(acct) => {
            acct.handler().await?;
        },
        Commands::System(system) => {
            system.handler(ctx).await?;
        }
    };
    Ok(())
}