/*
    Appellation: taxum <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use tempsdk::{App, Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cnf = Settings::build()?;
    let app = App::new(cnf).with_tracing();
    app.run().await?;

    Ok(())
}
