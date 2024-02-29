#[macro_use] extern crate rocket;

use rocket::serde::json::Json;

mod valida_cpf;
mod models;
mod divide_zero;

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
    match divide_zero::divide(6, 3) {
        Ok(x) => println!("Resultado da divisão: {}", x),
        Err(err) => println!("Erro: {}", err), // Não é necessário chamar .Error() em uma String
    }
    
    rocket::build().mount("/", routes![index, valida_cpf_endpoint])
}
