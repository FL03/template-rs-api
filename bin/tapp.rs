/*
    Appellation: taxum <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use tempsdk::{App, Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cnf = dbg!(Settings::build()?);
    let app = App::new(cnf).with_tracing().init().await?;
    println!("{:?}", app.ctx().fetch_samples().await?);
    app.run().await?;

    Ok(())
}
