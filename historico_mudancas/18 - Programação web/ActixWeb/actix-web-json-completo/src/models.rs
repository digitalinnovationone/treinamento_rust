use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cliente {
    pub id: u32,
    pub nome: String,
    pub cpf: String,
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub mensagem: String,
}
