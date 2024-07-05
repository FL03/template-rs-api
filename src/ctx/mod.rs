/*
    Appellation: ctx <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::context::Context;

pub(crate) mod context;

use std::sync::Arc;

pub trait Ctx<S = ()>
where
    S: Clone + Send + Sync + 'static,
{
    type Config;

    fn cnf(&self) -> &Self::Config;

    fn into_shared(self) -> Arc<Self>;

    fn state(&self) -> S;
}

pub trait AxumCtx: Ctx + Sized {
    fn into_ext_shared(self: Arc<Self>) -> axum::Extension<Arc<Self>> {
        axum::Extension(self)
    }

    fn into_ext(self) -> axum::Extension<Self> {
        axum::Extension(self)
    }
}

pub(crate) mod utils {
    
    pub fn map_err<T>(err: T) -> T
    where
        T: std::fmt::Display,
    {
        tracing::error!("{err}");
        err
    }
}
