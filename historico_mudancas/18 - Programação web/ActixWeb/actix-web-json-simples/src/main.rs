use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use config::{ConfigError, Config, File};

// Estruturas de configuração permanecem inalteradas
#[derive(Debug, Deserialize)]
struct AppConfig {
    server: ServerConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

fn load_config() -> Result<AppConfig, ConfigError> {
    let mut cfg = Config::new();
    cfg.merge(File::with_name("Actix"))?;
    cfg.merge(config::Environment::with_prefix("APP"))?;
    cfg.try_into()
}

// Definindo uma estrutura para a resposta JSON
#[derive(Serialize)]
struct Message {
    mensagem: String,
}

// Handler ajustado para retornar um JSON
async fn index() -> impl Responder {
    HttpResponse::Ok().json(Message { mensagem: "oi".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = load_config().expect("Failed to load configuration.");
    let server_cfg = cfg.server;
    let address = format!("{}:{}", server_cfg.host, server_cfg.port);

    println!("Iniciando o servidor em http://{}", address);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind(&address)?
    .run()
    .await
}