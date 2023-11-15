use config::{Config, ConfigError};
use serde::{Deserialize};
use tracing::info;

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    provider: String,
    host: String,
    port: u32,
    user: String,
    password: String,
    db_name: String,
    pub max_conn: u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct Api {
    host: String,
    port: u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct RabbitMQ {
    host: String,
    port: u32,
    user: String,
    password: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    debug: bool,
    database: Option<Database>,
    deployment_api: Option<Api>,
    rabbit_mq: Option<RabbitMQ>
}

impl AppConfig {

    #[tracing::instrument]
    pub fn new(config_file: String) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name(&config_file))
            .build()?;

        info!("deserializing configuration file {}", config_file);
        s.try_deserialize()
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn database(&self) -> Database {
        self.database.clone().expect("database settings not found")
    }

    pub fn rabbit_mq(&self) -> RabbitMQ {
        self.rabbit_mq.clone().expect("rabbitmq settings not found")
    }

    pub fn deployment_api(&self) -> Api {
        self.deployment_api.clone().expect("deployment api settings not found")
    }
}

impl Database {
    pub fn uri(&self) -> String {
        match self.provider.as_str() {
            "postgres" =>  format!("postgres://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.db_name),
            _ => "".to_string()
        }
    }
}

impl RabbitMQ {
    pub fn uri(&self) -> String {
        format!("amqp://{}:{}@{}:{}", self.user, self.password, self.host, self.port)
    }
}

impl Api {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
