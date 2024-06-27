/*
    Appellation: platform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Initializer;
use crate::{Context, Server, Settings};
use std::sync::Arc;

pub struct Platform {
    pub(crate) ctx: Arc<Context>,
    pub(crate) server: Server,
}

impl Platform {
    pub fn new(cnf: Settings) -> Initializer {
        Initializer::new(cnf)
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }

    pub fn cnf(&self) -> &Settings {
        self.ctx.settings()
    }

    #[tracing::instrument(skip_all, name = "run", target = "app")]
    pub async fn run(self) -> std::io::Result<()> {
        self.server.serve().await
    }
}

/*
 ************* Implementations *************
*/

impl AsRef<Context> for Platform {
    fn as_ref(&self) -> &Context {
        &self.ctx
    }
}

impl AsRef<Settings> for Platform {
    fn as_ref(&self) -> &Settings {
        self.ctx.settings()
    }
}
