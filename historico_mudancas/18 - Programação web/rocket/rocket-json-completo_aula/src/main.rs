#[macro_use] extern crate rocket;

mod controllers;
mod model_views;
mod models;
mod servicos;
mod dtos;
mod middlewares;

use controllers::{ home_controller, recursos_controller, login_controller };

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(middlewares::auth_guard::JwtFairing)
        .mount("/", routes![
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
