/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{collect_configurations, LogLevel, LoggerConfig, Mode, ServerAddr, ServerConfig};
use config::builder::{ConfigBuilder, DefaultState};

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct Settings {
    pub logger: LoggerConfig,
    pub mode: Mode,
    pub server: ServerConfig,
}

impl Settings {
    pub fn debug() -> Self {
        Self {
            logger: LoggerConfig::default(),
            mode: Mode::Development,
            server: ServerConfig::default(),
        }
    }

    fn builder() -> Result<ConfigBuilder<DefaultState>, config::ConfigError> {
        // initialize the builder with default values
        let mut builder = config::Config::builder()
            .set_default("logger.level", LogLevel::info())?
            .set_default("mode", Mode::development())?
            .set_default("server.addr.host", super::LOCALHOST)?
            .set_default("server.addr.port", 8080)?;
        // add sources
        builder = builder
            .add_source(config::Environment::with_prefix("APP").separator("_"))
            .add_source(config::File::with_name(".config/default.config.toml"));
        // setup overrides
        builder = builder
            .set_override_option("mode", std::env::var("APPMODE").ok())?
            .set_override_option("server.addr.host", std::env::var("SERVER_HOST").ok())?
            .set_override_option("server.addr.port", std::env::var("SERVER_PORT").ok())?;
        Ok(builder)
    }

    pub fn build() -> Result<Self, config::ConfigError> {
        Self::builder()?.build()?.try_deserialize()
    }

    pub fn build_from_file(file: &str) -> Result<Self, config::ConfigError> {
        Self::builder()?
            .add_source(config::File::with_name(file))
            .build()?
            .try_deserialize()
    }

    pub fn build_from_pattern(pattern: &str) -> Result<Self, config::ConfigError> {
        Self::builder()?
            .add_source(collect_configurations(pattern, false))
            .build()?
            .try_deserialize()
    }

    pub const fn logger(&self) -> &LoggerConfig {
        &self.logger
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub const fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn server_mut(&mut self) -> &mut ServerConfig {
        &mut self.server
    }

    pub const fn server_addr(&self) -> &ServerAddr {
        &self.server().addr()
    }

    pub fn server_addr_mut(&mut self) -> &mut ServerAddr {
        self.server_mut().addr_mut()
    }
}

/*
 ************* Implementations *************
*/

impl core::fmt::Display for Settings {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}

unsafe impl core::marker::Send for Settings {}

unsafe impl core::marker::Sync for Settings {}
