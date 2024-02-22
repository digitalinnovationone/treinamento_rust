use actix_web::{HttpServer, App};

mod jwt;
mod middleware;
mod handlers;
mod models;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::load_config().expect("Failed to load configuration.");
    let server_cfg = cfg.server;
    let address = format!("{}:{}", server_cfg.host, server_cfg.port);

    println!("Starting server at http://{}", address);

    HttpServer::new(move || {
        App::new()
            .configure(handlers::config_app)
    })
    .bind(&address)?
    .run()
    .await
}
