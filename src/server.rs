/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::builder::ServerBuilder;

pub(crate) mod builder;

use crate::{ApiRouterTy, Context};

use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct Server {
    ctx: Arc<Context>,
    router: Router,
}

impl Server {
    pub fn new(ctx: Arc<Context>) -> Self {
        let router = ServerBuilder::new()
            .routes(ctx.clone())
            .serve_file("./assets/index.html")
            .with_tracing()
            .build();
        Self { ctx, router }
    }

    pub fn from_context(ctx: Context) -> Self {
        Self::new(Arc::new(ctx))
    }

    pub async fn listen(&self) -> std::io::Result<TcpListener> {
        self.cnf().bind().await
    }

    pub fn cnf(&self) -> &crate::config::ServerConfig {
        self.ctx().settings().server()
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }

    pub fn router(&self) -> ApiRouterTy {
        self.router.clone()
    }

    pub async fn serve(self) -> std::io::Result<()> {
        let listener = self.listen().await?;
        let router = self.router.into_make_service();

        axum::serve(listener, router)
            .with_graceful_shutdown(utils::shutdown())
            .await
    }

    pub fn with_context(self, ctx: Context) -> Self {
        Self {
            ctx: Arc::new(ctx),
            ..self
        }
    }

    pub fn with_router(self, router: Router) -> Self {
        Self { router, ..self }
    }
}

pub(crate) mod utils {
    pub async fn shutdown() {
        tokio::signal::ctrl_c()
            .await
            .expect("CTRL+C: shutdown failed");

        tracing::info!("Shutdown the server...");
    }
}
