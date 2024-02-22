use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cliente {
    pub id: u32,
    pub nome: String,
    pub cpf: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenApi {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub mensagem: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub senha: String,
}
