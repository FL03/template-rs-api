/*
    Appellation: taxum <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use template_rs_api::{App, Settings};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cnf = Settings::build()?;
    let app = App::from_config(cnf).with_tracing();
    app.run().await?;

    Ok(())
}
