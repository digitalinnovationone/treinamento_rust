use rocket::serde::json::Json;

use crate::model_views::home::Home;

#[get("/")]
pub fn index() -> Json<Home> {
    Json(Home { mensagem: "Bem vindo".to_string() })
}
