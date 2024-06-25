/*
    Appellation: template-rs-api <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::api::prelude::*;

pub(crate) mod api;

pub mod config;
pub mod routes;

use self::config::{Context, Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cnf = Settings::build()?;
    let ctx = Context::from_config(cnf.clone());
    println!("{ctx}");

    let listener = ctx.server().bind().await?;
    let service = routes::v0().into_make_service();
    axum::serve(listener, service).with_graceful_shutdown(api::shutdown()).await?;

    Ok(())
}

