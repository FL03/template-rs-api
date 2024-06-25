/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::prelude::*;

// pub(crate) mod application;
pub(crate) mod server;

pub(crate) mod utils {

    pub async fn shutdown() {
        tokio::signal::ctrl_c()
            .await
            .expect("CTRL+C: shutdown failed");

        tracing::info!("Shutdown the server...");
    }
}


pub(crate) mod prelude {
    pub use super::App;
    pub use super::Server;
    pub use super::utils::*;
}