/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::Settings;
use std::sync::{Arc, Mutex};

#[derive(
    Clone, Debug,
)]
pub struct Context {
    pub(crate) db: sqlx::AnyPool,
    pub(crate) settings: Settings,
}

impl Context {
    pub async fn from_config(settings: Settings) -> Self {
        let db = settings.database.connect().await;
        Self { db, settings }
    }

    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn into_ext_shared(self: Arc<Self>) -> axum::Extension<Arc<Self>> {
        axum::Extension(self)
    }

    pub fn into_ext(self) -> axum::Extension<Self> {
        axum::Extension(self)
    }

    pub fn into_shared(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub fn into_mut_shared(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    pub fn with_tracing(self) -> Self {
        self.settings().logger().init_tracing();
        self
    }
}

/*
 ************* Implementations *************
*/
impl AsRef<Settings> for Context {
    fn as_ref(&self) -> &Settings {
        &self.settings
    }
}

impl AsMut<Settings> for Context {
    fn as_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }
}

impl core::borrow::Borrow<Settings> for Context {
    fn borrow(&self) -> &Settings {
        self.settings.borrow()
    }
}

impl core::borrow::BorrowMut<Settings> for Context {
    fn borrow_mut(&mut self) -> &mut Settings {
        self.settings.borrow_mut()
    }
}

impl core::ops::Deref for Context {
    type Target = Settings;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl core::ops::DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.settings
    }
}

impl core::fmt::Display for Context {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&self.settings().to_string())
    }
}

unsafe impl core::marker::Send for Context {}

unsafe impl core::marker::Sync for Context {}
