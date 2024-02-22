use actix_web::{web, HttpResponse, Responder};
use crate::models::{Message, Cliente};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(index))
        .service(
            web::scope("/clientes")
                .route("", web::get().to(listar_clientes))
                .route("", web::post().to(criar_cliente))
                .route("/{id}", web::get().to(buscar_cliente))
                .route("/{id}", web::put().to(atualizar_cliente))
                .route("/{id}", web::delete().to(deletar_cliente))
        );
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
    let cli = cliente.into_inner();
    clientes.push(cli.clone());

    HttpResponse::Created().json(cli)
}

async fn buscar_cliente(path: web::Path<(String,)>) -> impl Responder {
    let id = &path.0;
    if let Some(cliente) = obter_lista_clientes().iter().find(|c| c.id.to_string() == *id) {
        HttpResponse::Ok().json(cliente)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn atualizar_cliente(path: web::Path<(String,)>, cliente: web::Json<Cliente>) -> impl Responder {
    let mut clientes = obter_lista_clientes();
    let id = &path.0;
    if let Some(c) = clientes.iter_mut().find(|c| c.id.to_string() == *id) {
        let new_client = cliente.into_inner();
        c.nome = new_client.nome;
        c.cpf = new_client.cpf;
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
