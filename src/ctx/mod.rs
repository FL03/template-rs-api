/*
    Appellation: ctx <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::context::Context;

pub(crate) mod context;

use std::sync::Arc;

pub trait Ctx: Clone + Send + Sync + 'static {
    type Config;
}

pub trait AxumCtx: Ctx + Sized {
    fn into_ext_shared(self: Arc<Self>) -> axum::Extension<Arc<Self>> {
        axum::Extension(self)
    }

    fn into_ext(self) -> axum::Extension<Self> {
        axum::Extension(self)
    }
}
