use rocket::serde::json::Json;
use rocket::response::status::{Custom, NotFound};
use rocket::http::Status;
use rocket::response::status;

use crate::models;
use crate::models::cliente::Cliente;
use crate::servicos::cliente_servico;

#[derive(Debug, serde::Deserialize)]
pub struct NovoClienteJson {
    nome: String,
    cpf: String,
}

// Implementando a função processar_novo_cliente
fn processar_novo_cliente(cliente: &NovoClienteJson) -> Result<(), Custom<String>> {
    if cliente.nome == "Cliente Teste" {
        Ok(())
    } else {
        Err(Custom(Status::BadRequest, "Informações inválidas".to_string()))
    }
}

// Implementando a função atualizar_cliente
fn atualizar_cliente(id: u32, cliente: &NovoClienteJson) -> Result<(), Custom<String>> {
    println!("{}", id);
    
    if cliente.nome == "Cliente Teste" {
        Ok(())
    } else {
        Err(Custom(Status::BadRequest, "Erro ao atualizar o cliente".to_string()))
    }
}

// Implementando a função excluir_cliente
fn excluir_cliente(id: u32) -> Result<(), Custom<String>> {
    if id == 1 {
        Ok(())
    } else {
        Err(Custom(Status::NotFound, "Cliente não encontrado".to_string()))
    }
}

// Endpoint para listar todos os clientes
#[get("/clientes")]
pub fn index() -> Json<Vec<Cliente>> {
    let clientes = cliente_servico::get_clientes();
    Json(clientes)
}


#[post("/clientes", data = "<cliente_data>")]
pub async fn create(cliente_data: Json<NovoClienteJson>) -> Result<status::Custom<Json<Cliente>>, Custom<String>> {
    let cliente = cliente_data.into_inner();
    
    println!("{}", cliente.nome);
    println!("{}", cliente.cpf);

    processar_novo_cliente(&cliente)?;

    let cliente_db = models::cliente::Cliente{
        id: 0,
        nome: cliente.nome,
        cpf: cliente.cpf
    };

    Ok(status::Custom(Status::Created, Json(cliente_db)))
}


// Endpoint para editar um cliente existente
#[get("/clientes/<id>")]
pub fn show(id: u32) -> Result<Json<Cliente>, NotFound<&'static str>> {
    let cliente = Cliente { id: id, nome: "Cliente 1".to_string(), cpf: "000.000.000-01".to_string() };
    if cliente.id == 0 {
        Err(NotFound("Cliente não encontrado"))
    } else {
        Ok(Json(cliente))
    }
}

#[put("/clientes/<id>", data = "<cliente_data>")]
pub async fn update(id: u32, cliente_data: Json<NovoClienteJson>) -> Result<Json<String>, Custom<String>> {
    let cliente = cliente_data.into_inner();
    
    println!("{}", cliente.nome);
    println!("{}", cliente.cpf);

    atualizar_cliente(id, &cliente)?;

    Ok(Json("Cliente atualizado com sucesso".to_string()))
}

// Endpoint para excluir um cliente
#[delete("/clientes/<id>")]
pub fn delete(id: u32) -> Result<status::NoContent, status::Custom<String>> {
    excluir_cliente(id)?;
    Ok(status::NoContent)
}