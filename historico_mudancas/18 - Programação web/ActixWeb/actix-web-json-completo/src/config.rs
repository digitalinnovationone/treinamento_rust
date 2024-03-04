use config::{ConfigError, Config, File};
use serde::Deserialize;
use actix_web::web;
use crate::handlers;
use crate::middleware::AuthMiddleware;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub jwt: JwtConfig
}

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let mut cfg = Config::new();
    cfg.merge(File::with_name("Actix"))?;
    cfg.try_into()
}

pub fn routes(cfg: &mut web::ServiceConfig){
    cfg.route("/", web::get().to(handlers::home));
    cfg.route("/logar", web::post().to(handlers::logar));
    cfg.service(
        web::scope("/clientes")
        .wrap(AuthMiddleware)
        .route("", web::get().to(handlers::listar_clientes))
        .route("", web::post().to(handlers::criar_clientes))
        .route("/{id}", web::get().to(handlers::buscar_cliente))
        .route("/{id}", web::put().to(handlers::atualizar_clientes))
        .route("/{id}", web::delete().to(handlers::apagar_clientes))

    );
}