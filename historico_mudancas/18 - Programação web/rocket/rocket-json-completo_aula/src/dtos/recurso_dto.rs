use rocket::serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RecursoDto {
    pub titulo: String,
    pub descricao: String,
}