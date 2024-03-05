#[macro_use] extern crate rocket;

use rocket::serde::json::Json;

mod validar_cnpj;
mod models;

#[get("/")]
fn index() -> Json<models::HomeResponse> {
    Json(models::HomeResponse { mensagem: "Api para validar CNPJ - /validar_cnpj?cnpj=123567".to_string() })
}

#[get("/validar_cnpj?<cnpj>")]
fn valida_cnpj_endpoint(cnpj: &str) -> Json<models::ApiResponse> {
    let valido = validar_cnpj::validar(&cnpj);
    Json(models::ApiResponse { valido })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, valida_cnpj_endpoint])
}
