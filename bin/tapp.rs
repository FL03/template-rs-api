/*
    Appellation: taxum <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use tempsdk::{Platform, Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cnf = dbg!(Settings::build()?);
    let app = Platform::new(cnf).with_tracing().init().await?;
    app.run().await?;

    Ok(())
}
