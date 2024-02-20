#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Cliente {
    id: i32,
    nome: String,
    cpf: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Home {
    mensagem: String,
}

#[get("/")]
fn home() -> Json<Home> {
    Json(Home { mensagem: "Bem vindo".to_string() })
}

#[get("/clientes")]
fn clientes() -> Json<Vec<Cliente>> {
    let clientes = vec![
        Cliente { id: 1, nome: "Cliente 1".to_string(), cpf: "123.456.789-00".to_string() },
        Cliente { id: 2, nome: "Cliente 2".to_string(), cpf: "987.654.321-00".to_string() },
    ];

    Json(clientes)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![home, clientes])
}
