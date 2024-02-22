use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub fn load_config() -> Result<AppConfig, config::ConfigError> {
    let mut cfg = config::Config::default();
    cfg.merge(config::File::with_name("Actix")).unwrap();
    cfg.try_into()
}
