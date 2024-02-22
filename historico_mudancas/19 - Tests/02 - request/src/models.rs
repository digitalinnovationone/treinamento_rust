use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse {
    pub valido: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HomeResponse {
    pub mensagem: String,
}