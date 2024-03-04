use rocket::serde::json::Json;
use crate::models::recurso::Recurso;
use crate::model_views::erro_json::ErroJson;
use crate::dtos::recurso_dto::RecursoDto;
use crate::servicos::recurso_servico;
use rocket::response::status;
use rocket::http::Status;

#[get("/recursos")]
pub fn index() -> Json<Vec<Recurso>> {
    let recursos = recurso_servico::lista_de_recursos();
    Json(recursos)
}

#[post("/recursos", data = "<recurso_dto_json>")]
pub fn criar(recurso_dto_json: Json<RecursoDto>) -> Result<status::Custom<Json<Recurso>>, status::Custom<Json<ErroJson>>> {
    let recurso_dto = recurso_dto_json.into_inner();

    match recurso_servico::cadastrar_recurso(recurso_dto){
        Ok(recurso) => Ok(status::Custom(Status::Created, Json(recurso))), 
        Err(str_erro) => Err(status::Custom(Status::BadRequest, Json(ErroJson{ mensagem: str_erro }))),
    }
}

#[put("/recursos/<id>", data = "<recurso_dto_json>")]
pub fn alterar(id: u32, recurso_dto_json: Json<RecursoDto>) -> Result<status::Custom<Json<Recurso>>, status::Custom<Json<ErroJson>>> {
    let recurso_dto = recurso_dto_json.into_inner();

    match recurso_servico::alterar_recurso(id, recurso_dto){
        Ok(recurso) => Ok(status::Custom(Status::Ok, Json(recurso))), 
        Err(str_erro) => Err(status::Custom(Status::BadRequest, Json(ErroJson{ mensagem: str_erro }))),
    }
}

#[get("/recursos/<id>")]
pub fn mostrar(id: u32) -> status::Custom<Json<Recurso>> {
    let recurso = recurso_servico::busca_por_id(id);
    status::Custom(Status::Ok, Json(recurso))
}

#[delete("/recursos/<id>")]
pub fn excluir(id: u32) -> Result<status::Custom<Json<()>>, status::Custom<Json<ErroJson>>> {

    match recurso_servico::apagar_recurso_por_id(id){
        Ok(_) => Ok(status::Custom(Status::NoContent, Json(()))),
        Err(str_erro) => Err(status::Custom(Status::BadRequest, Json(ErroJson{ mensagem: str_erro }))),
    }
}