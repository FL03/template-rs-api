/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::cli::{cmd::Commands, CommandLineInterface};
use crate::{api::Api, Context};
use clap::Parser;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::task::JoinHandle;

pub async fn handle_cli(mut api: Api, cli: CommandLineInterface) -> anyhow::Result<()> {
    if let Some(cmd) = cli.command() {
        match cmd {
            Commands::System(sys) => {
                if sys.up {
                    tracing::info!("Message Recieved: System initializing...");
                    api.spawn().await?;
                }
            }
        }
    }

    Ok(())
}

#[derive(Clone, Debug)]
pub struct Runtime {
    pub api: Arc<Api>,
    pub ctx: watch::Receiver<Arc<Context>>,
    pub cli: Arc<CommandLineInterface>,
}

impl Runtime {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self {
            api: Api::from(ctx.as_ref().clone()).into(),
            cli: CommandLineInterface::parse().into(),
            ctx: watch::channel(ctx.clone()).1,
        }
    }
    pub async fn handle(&self) -> JoinHandle<anyhow::Result<&Self>> {
        let rt = Arc::new(self.clone());

        tokio::spawn(async move {
            let api = rt.api.clone();
            let cli = rt.cli.clone();
            loop {
                // Watch for updates to the application context
                if rt.ctx.has_changed().expect("Context channel droppped...") {
                    let ctx = rt.ctx.borrow().clone();
                    tracing::info!("Context changed: {:?}", ctx);
                }

                handle_cli(api.as_ref().clone(), cli.as_ref().clone()).await?;
            }
        })
    }
    pub async fn handler(&self) -> anyhow::Result<&Self> {
        handle_cli(self.api.as_ref().clone(), self.cli.as_ref().clone()).await?;

        Ok(self)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::from(Context::default())
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
        write!(
            f,
            "{}",
            serde_json::json!({"ctx": self.ctx.borrow().as_ref().clone() })
        )
    }
}
