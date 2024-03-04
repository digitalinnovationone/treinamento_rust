use actix_web::{web, HttpResponse, Responder};
use crate::models::{Cliente, Message, Login, TokenApi};
use crate::jwt::{Claims, create_token};
use crate::services;
use crate::config;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn home() ->impl Responder {
    HttpResponse::Ok().json(Message { mensagem: "Olá API".to_string() })
}



pub async fn logar(login_json: web::Json<Login>) -> impl Responder {
    let login = login_json.into_inner();
    
    if login.email == "danilo@teste.com" && login.senha == "123456" {
        // Substitua "admin_id" pelo ID real do administrador
        let admin_id = "admin_id"; 
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() + 60 * 60; // Token expira em 1 hora

        let claims = Claims {
            sub: admin_id.to_string(),
            exp: expiration as usize,
        };

        let config = config::load_config().expect("Failed to load configuration.");
        let jwt_secret = config.jwt.secret.clone();

        match create_token(claims, &jwt_secret) {
            Ok(token) => return HttpResponse::Ok().json(TokenApi { 
                token: token 
            }),
            Err(_) => return HttpResponse::BadRequest().json(Message { 
                mensagem: "Erro ao gerar token".to_string() 
            }),
        }
    }

    HttpResponse::BadRequest().json(Message { 
        mensagem: "Email ou senha inválidos".to_string() 
    })
}




pub async fn listar_clientes() ->impl Responder {
    let clientes = services::obter_lista_clientes();
    HttpResponse::Ok().json(clientes)
}

pub async fn criar_clientes(cliente: web::Json<Cliente>) ->impl Responder {
    let mut clientes = services::obter_lista_clientes();
    let cli = cliente.into_inner();
    clientes.push(cli.clone());

    HttpResponse::Created().json(cli)
}

pub async fn buscar_cliente(path: web::Path<(String,)>) -> impl Responder {
    let clientes = services::obter_lista_clientes();
    if let Some(cliente) = clientes.iter().find(|c| c.id.to_string() == path.0) {
        HttpResponse::Ok().json(cliente)
    } else {
        HttpResponse::NotFound().json(Message { mensagem: "Cliente não encontrado".to_string() })
    }
}

pub async fn atualizar_clientes(path: web::Path<(String,)>, cliente: web::Json<Cliente>) ->impl Responder {
    let mut clientes = services::obter_lista_clientes();
    let id = &path.0;
    if let Some(c) = clientes.iter_mut().find(|c| c.id.to_string() == *id) {
        c.nome = cliente.nome.clone();
        c.cpf = cliente.cpf.clone();
        HttpResponse::Ok().json(c)
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub async fn apagar_clientes(path: web::Path<(String,)>) ->impl Responder {
    let mut clientes = services::obter_lista_clientes();
    let id = &path.0;

    if let Some(index) = clientes.iter().position(|c| c.id.to_string() == *id) {
        clientes.remove(index);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().json(Message { mensagem: "Cliente não encontrado".to_string() })
    }
}
