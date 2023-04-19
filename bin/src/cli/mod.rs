/*
   Appellation: cli <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::interface::*;

pub mod cmd;
mod interface;

pub fn cli() -> CommandLineInterface {
    CommandLineInterface::default()
}
