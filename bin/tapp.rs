/*
    Appellation: tapp <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use template_rs_api::{Platform, Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cnf = Settings::build()?;
    let app = Platform::new(cnf).with_tracing().init().await?;
    app.run().await?;

    Ok(())
}
