use rocket::serde::json::Json;
use crate::servicos::auth_servico::generate_token;
use rocket::http::Status;

use crate::model_views::home::Home;
use crate::model_views::login_data::LoginData;

#[post("/logar", format = "json", data = "<login_data>")]
pub fn logar(login_data: Json<LoginData>) -> Result<Json<Home>, Status> {
    let user_id = "user123"; 

    println!("{}", login_data.username);
    println!("{}", login_data.password);

    let token = generate_token(user_id);

    if token.is_empty() {
        Err(Status::InternalServerError)
    } else {
        Ok(Json(Home { mensagem: format!("Bem vindo. Seu token: {}", token) }))
    }
}
