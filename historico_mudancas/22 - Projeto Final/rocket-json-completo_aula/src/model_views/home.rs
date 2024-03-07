use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Home {
    pub mensagem: String,
    pub endpoints: Vec<String>
}