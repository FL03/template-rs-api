/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Server;
use crate::config::{Context, Settings};
use std::sync::Arc;

pub struct App {
    pub(crate) ctx: Arc<Context>,
    server: Server,
}

impl App {
    pub fn new(ctx: Arc<Context>) -> Self {
        let server = Server::new(ctx.clone());

        Self { ctx, server }
    }

    pub fn from_config(cnf: Settings) -> Self {
        let ctx = Context::from_config(cnf);
        Self::new(ctx.into_shared())
    }

    pub fn ctx(&self) -> &Arc<Context> {
        &self.ctx
    }

    pub fn with_tracing(self) -> Self {
        self.ctx.init_tracing();
        tracing::info!("Successfully initialized the tracing layers...");
        self
    }

    pub async fn serve(self) -> std::io::Result<()> {
        self.server.serve().await
    }
}

/*
 ************* Implementations *************
*/

impl From<Arc<Context>> for App {
    fn from(ctx: Arc<Context>) -> Self {
        Self::new(ctx)
    }
}

impl From<Context> for App {
    fn from(ctx: Context) -> Self {
        Self::new(ctx.into_shared())
    }
}

impl From<Settings> for App {
    fn from(cnf: Settings) -> Self {
        Self::from_config(cnf)
    }
}
