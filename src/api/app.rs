
use crate::config::{Context, Settings};
use std::sync::Arc;

pub struct App {
    pub(crate) ctx: Arc<Context>,
}

impl App {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self { ctx }
    }

    pub fn from_config(cnf: Settings) -> Self {
        Self {
            ctx: Arc::new(Context::from_config(cnf)),
        }
    }

    pub fn ctx(&self) -> &Arc<Context> {
        &self.ctx
    }

    pub async fn serve(&self) -> std::io::Result<()> {
        let ctx = self.ctx.clone();
        let app = crate::routes::_api(ctx);

        let listener = self.ctx().server().bind().await?;
        let router = app.into_make_service();
        let server = axum::serve(listener, router).with_graceful_shutdown(super::shutdown());

        server.await
    }
}