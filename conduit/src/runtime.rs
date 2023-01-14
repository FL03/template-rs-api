/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{Context, Settings};
use acme::prelude::{AsyncSpawnable, Session};
use clap::{arg, ArgAction, ArgMatches, Command};
use scsys::prelude::{AsyncResult, Contextual};
use std::sync::Arc;
use tokio::task::JoinHandle;

pub async fn handle() -> JoinHandle<AsyncResult> {
    tokio::spawn(async move { Ok(()) })
}


#[derive(Clone, Debug)]
pub struct Runtime {
    pub ctx: Arc<Context>,
    pub session: Session
}

impl Runtime {
    pub fn new(ctx: Arc<Context>) -> Self {
        let session = Session::default();
        Self { ctx, session }
    }
    pub async fn handler(&self) -> AsyncResult<&Self> {
        if let Some(_up) = self.matches().get_one::<bool>("up") {
            crate::api::Api::from(self.ctx.clone()).spawn().await?;
        }
        Ok(self)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(Arc::new(Context::default()))
    }
}

impl RuntimeCliSpec for Runtime {
    fn command(&self) -> Command {
        Command::new("rt")
            .about("Manage the system runtime")
            .arg(arg!(service: -s --service <SERVICE>).action(ArgAction::SetTrue))
    }
}

impl Contextual for Runtime {
    type Cnf = Settings;

    type Ctx = Context;

    fn context(&self) -> &Self::Ctx {
        self.ctx.as_ref()
    }
}

impl From<Arc<Context>> for Runtime {
    fn from(ctx: Arc<Context>) -> Self {
        Self::new(ctx)
    }
}

impl From<Context> for Runtime {
    fn from(ctx: Context) -> Self {
        Self::from(Arc::new(ctx))
    }
}

impl std::fmt::Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!({"ctx": self.ctx.as_ref()}))
    }
}

pub trait RuntimeCliSpec {
    fn base(&self, sc: Command) -> ArgMatches {
        crate::cli::base(sc)
    }
    fn command(&self) -> Command;
    fn matches(&self) -> ArgMatches {
        self.base(self.command())
    }
}
