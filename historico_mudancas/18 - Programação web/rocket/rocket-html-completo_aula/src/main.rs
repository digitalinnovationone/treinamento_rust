#[macro_use] extern crate rocket;

mod models;
mod servicos;
mod dtos;

mod controllers {
    pub mod home_controller;
    pub mod clientes_controller;
}

use rocket_dyn_templates::Template;
use controllers::home_controller;
use controllers::clientes_controller;
use rocket::fs::{FileServer, relative};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        home_controller::index,

        clientes_controller::index,
        clientes_controller::novo,
        clientes_controller::criar,
        clientes_controller::editar,
        clientes_controller::alterar,
        clientes_controller::excluir,
    ])
    .mount("/static", FileServer::from(relative!("static")))
    .attach(Template::fairing())
}
