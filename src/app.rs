/*
    Appellation: app <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::init::*;

pub(crate) mod init;

use crate::{Context, Server, Settings};
use std::sync::Arc;

pub struct App {
    pub(crate) ctx: Arc<Context>,
    server: Server,
}

impl App {
    pub fn new(cnf: Settings) -> Initializer {
        Initializer::new(cnf)
    }    

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }

    pub fn cnf(&self) -> &Settings {
        self.ctx.settings()
    }

    pub async fn run(self) -> std::io::Result<()> {
        self.server.serve().await
    }
}

/*
 ************* Implementations *************
*/

impl AsRef<Context> for App {
    fn as_ref(&self) -> &Context {
        &self.ctx
    }
}

impl AsRef<Settings> for App {
    fn as_ref(&self) -> &Settings {
        self.ctx.settings()
    }
}
