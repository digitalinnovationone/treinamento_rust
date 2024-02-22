use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde::Serialize;

use crate::model_views::home::Home;

#[get("/")]
pub fn index() -> Json<Home> {
    Json(Home { mensagem: "Bem vindo".to_string() })
}

#[derive(Serialize)]
pub struct Negado {
    mensagem: String,
}

#[get("/unauthorized")]
pub fn unauthorized() -> status::Custom<Json<Negado>> {
    let mensagem = Negado {
        mensagem: "Sem autorização para acessar esta área".to_string(),
    };
    status::Custom(Status::Unauthorized, Json(mensagem))
}