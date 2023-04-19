/*
   Appellation: api <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::{api::routes, Context};
use axum::Router;
use http::header::{HeaderName, AUTHORIZATION};
use hyper::server::conn::AddrIncoming;
use std::sync::Arc;
use tower_http::{
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

/// Simple function wrapper for [tokio::signal::ctrl_c]
async fn shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    tracing::info!("Signal received; initiating shutdown procedures...");
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Api {
    ctx: Arc<Context>,
}

impl Api {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self {
            ctx,
        }
    }
    pub async fn client(&self) -> axum::Router {
        Router::new()
            .merge(routes::api())
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                    .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                AUTHORIZATION,
            )))
            .layer(CompressionLayer::new())
            .layer(PropagateHeaderLayer::new(HeaderName::from_static(
                "x-request-id",
            )))
            .layer(axum::Extension(self.ctx.clone()))
    }
    pub fn context(&self) -> Context {
        self.ctx.as_ref().clone()
    }
    /// Fetch an instance of a [std::net::SocketAddr]
    pub fn address(&self) -> std::net::SocketAddr {
        self.context().cnf.server.address()
    }
    /// Creates a new builder instance with the address created from the given port
    fn builder(&self) -> hyper::server::Builder<AddrIncoming> {
        tracing::debug!("Initializing the server");
        hyper::Server::bind(&self.address())
    }
    pub async fn serve(&self) -> anyhow::Result<()> {
        tracing::info!("Starting the server...");
        self.builder()
            .serve(self.client().awaitclient.into_make_service())
            .with_graceful_shutdown(shutdown())
            .await?;
        Ok(())
    }
    pub async fn spawn(&mut self) -> anyhow::Result<&Self> {
        self.server().serve(self.client().await).await?;
        Ok(self)
    }
}

impl From<Arc<Context>> for Api {
    fn from(ctx: Arc<Context>) -> Self {
        Self::new(ctx)
    }
}

impl From<Context> for Api {
    fn from(ctx: Context) -> Self {
        Self::from(Arc::new(ctx))
    }
}

impl std::fmt::Display for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ctx.settings().server().address())
    }
}

