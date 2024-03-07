use actix_web::{web, HttpResponse, Responder};
use crate::models::{Cliente, Message, Login, TokenApi};
use crate::jwt::{Claims, create_token};
use crate::utils;
use crate::repositorio;
use crate::models;
use crate::config;
use std::time::{SystemTime, UNIX_EPOCH};
use mysql::{Error as MySQLError};

pub async fn home() ->impl Responder {
    HttpResponse::Ok().json(Message { mensagem: "Olá API".to_string() })
}



pub async fn logar(login_json: web::Json<Login>) -> impl Responder {
    let login = login_json.into_inner();

    let email = utils::remove_sql_injection(login.email);
    let senha = utils::remove_sql_injection(login.senha);

    let adms = match repositorio::filtrar::<models::Administrador>(format!(" where email='{}' and senha='{}'", email, senha)) {
        Ok(adms) => adms,
        Err(error) => return error_response(error),
    };

    if !adms.is_empty() {
        let admin_id = adms[0].id; 
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


fn error_response(error: MySQLError) -> HttpResponse {
    let message = format!("Erro ao listar clientes: {}", error);
    HttpResponse::InternalServerError().json(message)
}


pub async fn listar_clientes() ->impl Responder {
    let clientes = match repositorio::listar::<Cliente>() {
        Ok(clientes) => clientes,
        Err(error) => return error_response(error),
    };

    HttpResponse::Ok().json(clientes)
}

pub async fn criar_clientes(cliente: web::Json<Cliente>) ->impl Responder {
    let cli = cliente.into_inner();

    match repositorio::inserir::<Cliente>(&cli) {
        Ok(_) => return HttpResponse::Ok().json(cli),
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
}

pub async fn buscar_cliente(path: web::Path<(String,)>) -> impl Responder {
    let clientes = match repositorio::filtrar::<models::Cliente>(format!(" where id={}",  path.0)) {
        Ok(clientes) => clientes,
        Err(error) => return error_response(error),
    };

    if !clientes.is_empty() {
        let cliente = &clientes[0];
        return HttpResponse::Ok().json(cliente);
    }

    HttpResponse::NotFound().json(Message { mensagem: "Cliente não encontrado".to_string() })
}

pub async fn atualizar_clientes(path: web::Path<(i32,)>, cliente: web::Json<Cliente>) -> impl Responder {
    let id = path.0;
    let cli = cliente.into_inner();

    match repositorio::atualizar::<Cliente>(id, &cli) {
        Ok(_) => HttpResponse::Ok().json(cli),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn apagar_clientes(path: web::Path<(i32,)>) ->impl Responder {
    let id = path.0;

    match repositorio::excluir::<Cliente>(id) {
        Ok(_) => return HttpResponse::NoContent().finish(),
        Err(_) => return HttpResponse::NotFound().json(Message { mensagem: "Cliente não encontrado".to_string() }),
    };
}
