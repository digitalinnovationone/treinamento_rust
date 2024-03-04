use actix_web::{App, HttpServer};

mod config;
mod models;
mod handlers;
mod services;
mod middleware;
mod jwt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::load_config().expect("Failed to load configuration.");
    let server_cfg = cfg.server;
    let address = format!("{}:{}", server_cfg.host, server_cfg.port);

    println!("Iniciando o servidor em http://{}", address);

    HttpServer::new(|| {
        App::new().configure(config::routes)

    })
    .bind(&address)?
    .run()
    .await
}
