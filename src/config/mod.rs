/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub(crate) use self::utils::*;
pub use self::{kinds::*, settings::*};

pub(crate) mod settings;

mod kinds {
    #[doc(inline)]
    pub use self::prelude::*;

    pub(crate) mod logger;
    pub(crate) mod mode;
    pub(crate) mod server;

    pub(crate) mod prelude {
        pub use super::logger::*;
        pub use super::mode::*;
        pub use super::server::*;
    }
}

pub(crate) mod utils {
    use std::path::PathBuf;

    /// Type alias for [config::File]
    pub(crate) type ConfigFile<Src, Fmt> = config::File<Src, Fmt>;
    /// Type alias for a collection of [crate::ConfigFile]
    pub(crate) type ConfigFileVec = Vec<ConfigFile<config::FileSourceFile, config::FileFormat>>;

    /// A generic function wrapper extending glob::glob
    fn gather<F, T>(pattern: &str, f: F) -> Vec<T>
    where
        F: Copy + Fn(PathBuf) -> T,
    {
        glob::glob(pattern)
            .expect("Failed to collect files")
            .filter_map(|r| r.ok().map(f))
            .collect()
    }

    /// Attempts to collect configuration files, following the given pattern, into a ConfigFileVec
    pub fn collect_configurations(pattern: &str, required: bool) -> ConfigFileVec {
        gather(pattern, |p| ConfigFile::from(p).required(required))
    }
}
