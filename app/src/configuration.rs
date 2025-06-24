use pavex::blueprint::Blueprint;
use pavex::server::IncomingStream;
use pavex::t;
use pavex::time::SignedDuration;
use pavex_session_sqlx::SqliteSessionStore;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::ConnectOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};

/// Refer to Pavex's [configuration guide](https://pavex.dev/docs/guide/configuration) for more details
/// on how to manage configuration values.
pub fn register(bp: &mut Blueprint) {
    bp.config("server", t!(self::ServerConfig));
    bp.config("database", t!(self::DatabaseConfig));
    bp.singleton(pavex::f!(self::DatabaseConfig::get_pool));
}

#[derive(serde::Deserialize, Debug, Clone)]
/// Configuration for the HTTP server used to expose our API
/// to users.
pub struct ServerConfig {
    /// The port that the server must listen on.
    ///
    /// Set the `PX_SERVER__PORT` environment variable to override its value.
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub port: u16,
    /// The network interface that the server must be bound to.
    ///
    /// E.g. `0.0.0.0` for listening to incoming requests from
    /// all sources.
    ///
    /// Set the `PX_SERVER__IP` environment variable to override its value.
    pub ip: std::net::IpAddr,
    /// The timeout for graceful shutdown of the server.
    ///
    /// E.g. `1 minute` for a 1 minute timeout.
    ///
    /// Set the `PX_SERVER__GRACEFUL_SHUTDOWN_TIMEOUT` environment variable to override its value.
    #[serde(deserialize_with = "deserialize_shutdown")]
    pub graceful_shutdown_timeout: std::time::Duration,
}

fn deserialize_shutdown<'de, D>(deserializer: D) -> Result<std::time::Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let duration = SignedDuration::deserialize(deserializer)?;
    if duration.is_negative() {
        Err(serde::de::Error::custom(
            "graceful shutdown timeout must be positive",
        ))
    } else {
        duration.try_into().map_err(serde::de::Error::custom)
    }
}

impl ServerConfig {
    /// Bind a TCP listener according to the specified parameters.
    pub async fn listener(&self) -> Result<IncomingStream, std::io::Error> {
        let addr = std::net::SocketAddr::new(self.ip, self.port);
        IncomingStream::bind(addr).await
    }
}

// struct type to represent the database configuration
#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub database_path: String,
}

// methods for the database configuration type
impl DatabaseConfig {
    pub fn connect_options(&self) -> SqliteConnectOptions {
        SqliteConnectOptions::new()
            .filename(&self.database_path)
            .create_if_missing(true)
            .log_statements(tracing_log::log::LevelFilter::Trace)
    }

    pub async fn get_pool(&self) -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(self.connect_options());

        // Run migrations
        let store = SqliteSessionStore::new(pool.clone());
        store
            .migrate()
            .await
            .expect("Failed to run SQLite migrations");

        pool
    }
}
