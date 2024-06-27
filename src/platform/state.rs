/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub struct PlatformState {
    pub(crate) db: sqlx::AnyPool,
}
