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


use std::{convert::From, sync::Arc};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create an application instance
    let mut app = Application::default();
    // Quickstart the application runtime with the following command
    app.setup()?;
    app.spawn().await?;

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
        let state = State::default();

        Self {
            ctx: ctx.clone(),
            rt: Arc::new(runtime::Runtime::from(ctx)),
            state: state.into(),
        }
    }
    /// Application runtime
    pub fn runtime(&self) -> &runtime::Runtime {
        self.rt.as_ref()
    }
    pub async fn spawn(&mut self) -> anyhow::Result<&Self> {
        let ctx_chan = tokio::sync::watch::channel(self.ctx.clone());
        ctx_chan
            .0
            .send(self.ctx.clone())
            .expect("Context channel droppped...");

        let state_chan = tokio::sync::watch::channel(self.state.clone());
        state_chan
            .0
            .send(self.state.clone())
            .expect("State channel droppped...");

        tracing::debug!("Spawning the application and related services...");
        self.state = State::Process.into();
        state_chan
            .0
            .send(self.state.clone())
            .expect("State channel droppped...");
        // Fetch the initialized cli and process the results
        self.runtime().handler().await?;
        // Signal process completion with a change of state
        self.state = State::Complete.into();
        state_chan
            .0
            .send(self.state.clone())
            .expect("State channel droppped...");
        // Resume default application behaviour
        self.state = State::Idle.into();
        state_chan
            .0
            .send(self.state.clone())
            .expect("State channel droppped...");
        Ok(self)
    }

    pub fn init(self) -> Self {
        self
    }

    pub fn context(&self) -> Context {
        self.ctx.clone()
    }

    pub fn name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    pub fn settings(&self) -> Settings {
        self.ctx.settings().clone()
    }

    pub fn setup(&mut self) -> anyhow::Result<&Self> {
        self.settings().logger().clone().setup(None);
        tracing_subscriber::fmt::init();
        tracing::debug!("Application initialized; completing setup...");
        Ok(self)
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
