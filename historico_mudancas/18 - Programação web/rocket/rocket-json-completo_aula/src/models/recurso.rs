use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Recurso {
    pub id: u32,
    pub titulo: String,
    pub descricao: String,
}