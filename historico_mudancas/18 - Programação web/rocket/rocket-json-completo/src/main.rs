#[macro_use] extern crate rocket;

mod model_views;
mod models;
mod servicos;

mod controllers {
    pub mod home_controller;
    pub mod clientes_controller;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            controllers::home_controller::index,

            controllers::clientes_controller::index,
            controllers::clientes_controller::create,
            controllers::clientes_controller::update,
            controllers::clientes_controller::show,
            controllers::clientes_controller::delete,
        ])
}
