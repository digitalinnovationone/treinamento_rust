#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use serde::Serialize; // Importação correta de Serialize

#[derive(Serialize)] // Garante que Cliente possa ser serializado
struct Cliente {
    id: u32,
    nome: String,
    cpf: String,
}

fn get_clientes() -> Vec<Cliente> {
    vec![
        Cliente { id: 1, nome: "Cliente 1".to_string(), cpf: "000.000.000-01".to_string() },
        Cliente { id: 2, nome: "Cliente 2".to_string(), cpf: "000.000.000-02".to_string() },
    ]
}

#[get("/clientes")]
fn clientes() -> Template {
    let clientes = get_clientes(); // Obtem a lista de clientes
    Template::render("clientes", context! { clientes: &clientes })
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, clientes])
        .attach(Template::fairing())
}
