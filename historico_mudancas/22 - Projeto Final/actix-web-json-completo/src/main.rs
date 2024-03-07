#[macro_use]
extern crate model_macro;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use actix_web::middleware::Logger;
mod config;
mod utils;
mod models;
mod handlers;
mod middleware;
mod repositorio;
mod jwt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*
    // o migrate que cria a tabela
    repositorio::criar_tabela::<models::Cliente>().expect("Falha ao criar tabela");
    repositorio::criar_tabela::<models::Administrador>().expect("Falha ao criar tabela");

    // seed dos dados
    let adm = models::Administrador {
        id: 0,
        nome: "Danilo".to_string(),
        email: "danilo@teste.com".to_string(),
        senha: "123456".to_string(),
    };
    repositorio::inserir::<models::Administrador>(&adm).expect("Falha incluir administrador");
    */

    let cfg = config::load_config().expect("Failed to load configuration.");
    let server_cfg = cfg.server;
    let address = format!("{}:{}", server_cfg.host, server_cfg.port);

    println!("Iniciando o servidor em http://{}", address);

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).configure(config::routes)

    })
    .bind(&address)?
    .run()
    .await
}
