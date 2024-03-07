use rocket::serde::json::Json;
use crate::model_views::home::Home;
use crate::model_views::erro_json::ErroJson;
use rocket::response::status;
use rocket::http::Status;

#[get("/")]
pub fn index() -> Json<Home> {
    Json(Home{ 
        mensagem: "bem vindo a API".to_string(),
        endpoints: vec![
            "/recursos".to_string()
        ]
    })
}

#[get("/nao-autorizado")]
pub fn nao_autorizado() -> status::Custom<Json<ErroJson>> {
    status::Custom(Status::Unauthorized, Json(ErroJson {
        mensagem: "Sem autorização para acessar esta área".to_string(),
    }))
}