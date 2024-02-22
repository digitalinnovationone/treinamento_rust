use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Cliente {
    pub id: u32,
    pub nome: String,
    pub cpf: String,
}
