#[macro_use] extern crate rocket;

mod model_views;
mod models;
mod servicos;
mod middlewares;

mod controllers {
    pub mod home_controller;
    pub mod clientes_controller;
    pub mod login_controller;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(middlewares::auth_guard::JwtFairing)
        .mount("/", routes![
            controllers::home_controller::index,
            controllers::home_controller::unauthorized,

            controllers::login_controller::logar,

            controllers::clientes_controller::index,
            controllers::clientes_controller::create,
            controllers::clientes_controller::update,
            controllers::clientes_controller::show,
            controllers::clientes_controller::delete,
        ])
}
