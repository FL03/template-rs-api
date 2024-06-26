/*
    Appellation: database <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

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
pub struct DbConfig {
    pub url: String,
    pub pool_size: Option<u32>,
}

impl DbConfig {
    pub async fn connect<Db>(&self) -> sqlx::Result<sqlx::Pool<Db>>
    where
        Db: sqlx::Database,
    {
        tracing::debug!("Connecting to database: {}", self.url);
        sqlx::Pool::connect(&self.url).await
    }
}

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
pub struct Uri {
    pub prefix: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl Uri {
    pub fn new(
        prefix: impl ToString,
        host: impl ToString,
        port: u16,
        user: impl ToString,
        password: impl ToString,
        database: impl ToString,
    ) -> Self {
        Self {
            prefix: prefix.to_string(),
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            database: database.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{prefix}://{user}:{password}@{host}:{port}/{database}",
            prefix = self.prefix,
            user = self.user,
            password = self.password,
            host = self.host,
            port = self.port,
            database = self.database
        )
    }
}

impl core::fmt::Display for Uri {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&self.to_string())
    }
}
