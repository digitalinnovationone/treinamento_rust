use crate::models::recurso::Recurso;
use crate::dtos::recurso_dto::RecursoDto;
use crate::repositorios::recurso_repositorio;
use crate::config::cnn;

pub fn lista_de_recursos() -> Vec<Recurso> {
    let conn = cnn::establish_connection();
    return match recurso_repositorio::listar(&conn) {
        Ok(recursos) => recursos,
        Err(error) => {
            println!("{}", error);
            Vec::new()
        },
    };
}

pub fn buscar_por_id(id: u32) -> Option<Recurso> {
    let conn = cnn::establish_connection();
    match recurso_repositorio::buscar_por_id(&conn, id) {
        Ok(recurso) => recurso,
        Err(error) => {
            println!("Erro ao buscar recurso: {}", error);
            None
        },
    }
}

pub fn apagar_recurso_por_id(id: u32) -> Result<(), String> {
    let conn = cnn::establish_connection();
    match recurso_repositorio::excluir(&conn, id) {
        Ok(_) => return Ok(()),
        Err(error) => return Err(format!("Erro ao atualizar - {}", error)),
    }
}

pub fn cadastrar_recurso(recurso_dto: RecursoDto) -> Result<Recurso, String> {
    if recurso_dto.titulo == ""{
        return Err("O titulo não pode ser vazio".to_string())
    }

    let conn = cnn::establish_connection();
    match recurso_repositorio::criar(&conn, &recurso_dto.titulo, &recurso_dto.descricao) {
        Ok(_) => return Ok(Recurso { id: 0, titulo: recurso_dto.titulo, descricao: Some(recurso_dto.descricao) }),
        Err(error) => return Err(format!("Erro ao atualizar - {}", error)),
    }
}

pub fn alterar_recurso(id:u32, recurso_dto: RecursoDto) -> Result<Recurso, String> {
    if recurso_dto.titulo == ""{
        return Err("O titulo não pode ser vazio".to_string())
    }

    let conn = cnn::establish_connection();
    match recurso_repositorio::atualizar(&conn, id, &recurso_dto.titulo, &recurso_dto.descricao) {
        Ok(_) => return Ok(Recurso { id: id, titulo: recurso_dto.titulo, descricao: Some(recurso_dto.descricao) }),
        Err(error) => return Err(format!("Erro ao atualizar - {}", error)),
    }
}