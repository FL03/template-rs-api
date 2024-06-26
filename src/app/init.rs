/*
    Appellation: init <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{App, Settings};

pub struct Initializer {
    cnf: Settings,
}

impl Initializer {
    pub fn new(cnf: Settings) -> Self {
        Self { cnf }
    }

    pub fn init(self) -> App {
        App::new(self.cnf)
    }
}
