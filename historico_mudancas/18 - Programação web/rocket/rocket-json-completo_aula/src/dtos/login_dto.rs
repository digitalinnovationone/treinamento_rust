use rocket::serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginDto {
    pub email: String,
    pub senha: String,
}