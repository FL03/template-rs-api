/*
    Appellation: models <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::items::ItemModel;

pub mod items;

pub type LocalTz = chrono::Local;
pub type Timestamp<Tz = chrono::Utc> = chrono::DateTime<Tz>;

pub type ItemId = uuid::Uuid;

pub type BigInt = i64;

pub type BigSerial = BigInt;
