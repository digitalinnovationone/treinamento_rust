#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};

mod models;
mod servicos;

mod controllers {
    pub mod home_controller;
    pub mod clientes_controller;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/", routes![
            controllers::home_controller::index,

            controllers::clientes_controller::index,
            controllers::clientes_controller::novo,
            controllers::clientes_controller::editar,
            controllers::clientes_controller::cadastrar,
            controllers::clientes_controller::alterar,
            controllers::clientes_controller::excluir,
        ])
}
