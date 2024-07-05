/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused_imports)]
pub use self::consts::*;

pub mod consts {
    pub const APP_NAME: &str = "tapp"; // env!("CARGO_PKG_NAME");
    pub const ENV_PREFIX: &str = "APP";
}
