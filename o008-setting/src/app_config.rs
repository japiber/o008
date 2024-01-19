use config::{Config, ConfigError};
use serde::{Deserialize};

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
pub struct Bus {
    response_capacity: usize,
    request_capacity: usize,
    response_wait: u64,
    request_wait: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    debug: bool,
    database: Option<Database>,
    deployment_api: Option<Api>,
    bus: Option<Bus>
}

impl AppConfig {

    #[tracing::instrument]
    pub fn new(config_file: String) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name(&config_file))
            .build()?;

        s.try_deserialize()
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn database(&self) -> Database {
        self.database.clone().expect("database settings not found")
    }

    pub fn bus(&self) -> Bus {
        self.bus.clone().expect("bus sttings not found")
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

impl Bus {
    pub fn response_capacity(&self) -> usize {
       self.response_capacity
    }

    pub fn request_capacity(&self) -> usize {
        self.request_capacity
    }

    pub fn response_wait(&self) -> u64 {
        self.response_wait
    }

    pub fn request_wait(&self) -> u64 {
        self.request_wait
    }
}

impl Api {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
