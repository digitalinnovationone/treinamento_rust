use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct Cliente {
    id: Uuid,
    nome: String,
    cpf: String,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    server: ServerConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Serialize)]
struct Message {
    mensagem: String,
}

#[derive(Debug)]
struct AppState {
    clientes: Arc<Mutex<Vec<Cliente>>>,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().json(Message { mensagem: "Olá! Bem-vindo à API de clientes.".to_string() })
}

async fn listar_clientes() -> impl Responder {
    let clientes = obter_lista_clientes();
    HttpResponse::Ok().json(clientes)
}

async fn criar_cliente(cliente: web::Json<Cliente>) -> impl Responder {
    let mut clientes = obter_lista_clientes();
    clientes.push(cliente.into_inner());
    HttpResponse::Created().finish()
}

async fn buscar_cliente(path: web::Path<(String,)>) -> impl Responder {
    let clientes = obter_lista_clientes();
    if let Some(cliente) = clientes.iter().find(|c| c.id.to_string() == path.0) {
        HttpResponse::Ok().json(cliente)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn atualizar_cliente(path: web::Path<(String,)>, cliente: web::Json<Cliente>) -> impl Responder {
    let mut clientes = obter_lista_clientes();
    if let Some(mut c) = clientes.iter_mut().find(|c| c.id.to_string() == path.0) {
        c.nome = cliente.nome.clone();
        c.cpf = cliente.cpf.clone();
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn deletar_cliente(path: web::Path<(String,)>) -> impl Responder {
    let mut clientes = obter_lista_clientes();
    if let Some(index) = clientes.iter().position(|c| c.id.to_string() == path.0) {
        clientes.remove(index);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

fn obter_lista_clientes() -> Vec<Cliente> {
    // Implemente aqui a lógica para obter a lista de clientes.
    // Neste exemplo, uma lista vazia é retornada.
    Vec::new()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = load_config().expect("Failed to load configuration.");
    let server_cfg = cfg.server;
    let address = format!("{}:{}", server_cfg.host, server_cfg.port);

    println!("Iniciando o servidor em http://{}", address);

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/clientes", web::get().to(listar_clientes))
            .route("/clientes", web::post().to(criar_cliente))
            .route("/clientes/{id}", web::get().to(buscar_cliente))
            .route("/clientes/{id}", web::put().to(atualizar_cliente))
            .route("/clientes/{id}", web::delete().to(deletar_cliente))
    })
    .bind(&address)?
    .run()
    .await
}

fn load_config() -> Result<AppConfig, config::ConfigError> {
    let mut cfg = config::Config::default();
    cfg.merge(config::File::with_name("config")).unwrap();
    cfg.try_into()
}