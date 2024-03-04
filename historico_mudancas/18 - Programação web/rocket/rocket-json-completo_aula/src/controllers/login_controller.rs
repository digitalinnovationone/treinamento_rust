use rocket::serde::json::Json;
use crate::model_views::administrador_token::AdministradorToken;
use crate::model_views::erro_json::ErroJson;
use crate::dtos::login_dto::LoginDto;
use crate::servicos::administrador_servico;
use rocket::response::status;
use rocket::http::Status;

#[post("/login", data = "<login_dto_json>")]
pub fn login(login_dto_json: Json<LoginDto>) -> Result<status::Custom<Json<AdministradorToken>>, status::Custom<Json<ErroJson>>> {
    let login_dto = login_dto_json.into_inner();

    match administrador_servico::login(login_dto.email, login_dto.senha){
        Ok(administrador_token) => Ok(status::Custom(Status::Created, Json(administrador_token))), 
        Err(str_erro) => Err(status::Custom(Status::BadRequest, Json(ErroJson{ mensagem: str_erro }))),
    }
}