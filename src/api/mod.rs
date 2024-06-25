/*
    Appellation: api <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::prelude::*;

pub(crate) mod app;

pub(crate) mod utils {

    pub async fn shutdown() {
        tokio::signal::ctrl_c()
            .await
            .expect("CTRL+C: shutdown failed");

        tracing::info!("Shutdown the server...");
    }
}

pub(crate) mod prelude {
    pub use super::app::App;
    pub use super::utils::*;
}