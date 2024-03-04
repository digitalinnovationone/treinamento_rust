use serde::Serialize;

#[derive(Serialize)]
pub struct Cliente {
    pub id: u32,
    pub nome: String,
    pub cpf: String,
}
