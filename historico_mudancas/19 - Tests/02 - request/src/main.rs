#[macro_use] extern crate rocket;

use rocket::serde::json::Json;

mod valida_cpf;
mod models;

#[get("/")]
fn index() -> Json<models::HomeResponse> {
    Json(models::HomeResponse { mensagem: "Api valida CPF - /valida_cpf?cpf=123567".to_string() })
}

#[get("/valida_cpf?<cpf>")]
fn valida_cpf_endpoint(cpf: &str) -> Json<models::ApiResponse> {
    let valido = valida_cpf::valida_cpf(&cpf);
    Json(models::ApiResponse { valido })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, valida_cpf_endpoint])
}
