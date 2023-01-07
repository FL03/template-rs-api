/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::AsyncResult;

pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {

        Self
    }

    pub async fn runtime(&self) -> AsyncResult {
        loop {

        }
    }
}