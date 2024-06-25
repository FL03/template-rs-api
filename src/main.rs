/*
    Appellation: template-rs-api <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod config;
pub mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cnf = config::Settings::build()?;
    println!("{cnf}");

    Ok(())
}

