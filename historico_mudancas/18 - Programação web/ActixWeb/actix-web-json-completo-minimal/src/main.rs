use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Cliente {
    id: u32,
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

async fn index() -> impl Responder {
    HttpResponse::Ok().json(Message { mensagem: "Olá! Bem-vindo a API de clientes.".to_string() })
}

async fn listar_clientes() -> impl Responder {
    let clientes = obter_lista_clientes();
    HttpResponse::Ok().json(clientes)
}

async fn criar_cliente(cliente: web::Json<Cliente>) -> impl Responder {
    let mut clientes = obter_lista_clientes();
    let cli = cliente.into_inner();
    clientes.push(cli.clone());

    HttpResponse::Created().json(cli)
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
    let id = &path.0;
    if let Some(c) = clientes.iter_mut().find(|c| c.id.to_string() == *id) {
        c.nome = cliente.nome.clone();
        c.cpf = cliente.cpf.clone();
        HttpResponse::Ok().json(c)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn deletar_cliente(path: web::Path<(String,)>) -> impl Responder {
    let mut clientes = obter_lista_clientes();
    let id = &path.0;

    if let Some(index) = clientes.iter().position(|c| c.id.to_string() == *id) {
        clientes.remove(index);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

fn obter_lista_clientes() -> Vec<Cliente> {
    let cliente1 = Cliente {
        id: 1,
        nome: String::from("Cliente 1"),
        cpf: String::from("111.111.111-11"),
    };
    
    let cliente2 = Cliente {
        id: 2,
        nome: String::from("Cliente 2"),
        cpf: String::from("222.222.222-22"),
    };
    
    let mut lista_clientes = Vec::new();
    lista_clientes.push(cliente1);
    lista_clientes.push(cliente2);
    
    lista_clientes
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = load_config().expect("Falha ao carregar a configuração. Verifique se o arquivo de configuração existe e está correto.");
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
    cfg.merge(config::File::with_name("Actix")).unwrap();
    cfg.try_into()
}
