use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable)]
pub struct Recurso {
    pub id: u32,
    pub titulo: String,
    pub descricao: Option<String>,
}