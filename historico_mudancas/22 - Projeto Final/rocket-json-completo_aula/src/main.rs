#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod controllers;
mod model_views;
mod models;
mod servicos;
mod dtos;
mod middlewares;
mod config;
mod repositorios;
mod schema;

use controllers::{ home_controller, recursos_controller, login_controller };
use rocket::{http::Status, response::status};

#[options("/<_..>")]
fn all_options() -> status::Custom<&'static str> {
    status::Custom(
        Status::NoContent,
        ""
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(middlewares::cors::CorsFairing)
        .attach(middlewares::auth_guard::JwtFairing)
        .mount("/", routes![
            all_options,

            home_controller::index,
            home_controller::nao_autorizado,
            login_controller::login,

            recursos_controller::index,
            recursos_controller::criar,
            recursos_controller::alterar,
            recursos_controller::mostrar,
            recursos_controller::excluir,
        ]
    )
}
