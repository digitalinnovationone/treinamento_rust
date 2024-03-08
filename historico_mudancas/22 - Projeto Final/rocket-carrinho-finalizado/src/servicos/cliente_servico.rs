use crate::config::cnn::establish_connection;
use crate::models::cliente::Cliente;
use crate::repositorios::cliente_repositorio;

pub fn listar() -> Vec<Cliente> {
    let conn = establish_connection();
    cliente_repositorio::listar(&conn).unwrap()
}

pub fn criar(nome: String, telefone: String) -> bool {
    let conn = establish_connection();
    cliente_repositorio::criar(&conn, &nome, &telefone).is_ok()
}

pub fn alterar(id: u32, nome: String, telefone: String) -> bool {
    let conn = establish_connection();
    cliente_repositorio::atualizar(&conn, id as u32, &nome, &telefone).is_ok()
}

pub fn excluir_por_id(id: u32) -> bool {
    let conn = establish_connection();
    cliente_repositorio::excluir(&conn, id as u32).is_ok()
}

pub fn buscar_por_id(id: u32) -> Cliente {
    let conn = establish_connection();
    cliente_repositorio::buscar_por_id(&conn, id as u32).unwrap_or_else(|_| Cliente {
        id: id as u32,
        nome: "Cliente não encontrado".to_string(),
        telefone: "".to_string(),
    })
}
