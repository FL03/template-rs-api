/*
    Appellation: init <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{App, Context, Settings, Server};

pub struct Initializer {
    cnf: Settings,
}

impl Initializer {
    pub fn new(cnf: Settings) -> Self {
        Self { cnf }
    }

    pub fn with_tracing(self) -> Self {
        self.cnf.logger().init_tracing();

        self
    }



    pub async fn init(self) -> App {
        let db = self.cnf.database.connect().await;
        let ctx = Context {
            db,
            settings: self.cnf,
        }.into_shared();
        let server = Server::new(ctx.clone());

        App {
            ctx,
            server,
        }
    }
}
