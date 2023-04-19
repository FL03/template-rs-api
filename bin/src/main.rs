/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{context::*, primitives::*, settings::*, states::*};

mod context;
mod primitives;
mod settings;
mod states;

pub mod api;
pub mod cli;
pub mod runtime;


use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = api::new();
    api.serve().await?;
    // Create an application instance
    // let mut app = Application::default().setup();
    // app.spawn().await?;

    Ok(())
}

#[derive(Clone, Debug)]
pub struct Application {
    pub ctx: Arc<Context>,
    pub rt: Arc<runtime::Runtime>,
    pub state: Locked<State>,
}

impl Application {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self {
            ctx: ctx.clone(),
            rt: Arc::new(runtime::Runtime::from(ctx)),
            state: Arc::new(Mutex::new(State::default())),
        }
    }
    /// Application runtime
    pub fn runtime(&self) -> &runtime::Runtime {
        self.rt.as_ref()
    }
    pub async fn spawn(&mut self) -> anyhow::Result<()> {
        Ok(loop {
            tokio::select! {
                _ = self.runtime().handle() => {
                    tracing::info!("Message Recieved: Shutting down...");
                    break;
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("Message Recieved: Shutting down...");
                    break;
                }
            }
        })
    }

    pub fn init(self) -> Self {
        self
    }

    pub fn name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    pub fn settings(&self) -> Settings {
        self.ctx.settings().clone()
    }
    pub fn setup(self) -> Self {
        // Initialize tracing layer...
        let logger = self.ctx.settings().logger.clone();
        logger.setup_env(None).init_tracing();
        self
    }

    pub fn state(&self) -> &Locked<State> {
        &self.state
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::from(Context::default())
    }
}

impl From<Settings> for Application {
    fn from(data: Settings) -> Self {
        Self::from(Context::from(data))
    }
}

impl From<Context> for Application {
    fn from(data: Context) -> Self {
        Self::new(Arc::new(data))
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.ctx.as_ref()).unwrap())
    }
}
