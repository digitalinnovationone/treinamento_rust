use serde::Serialize;

#[derive(Serialize)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub descricao: String,
    pub imagem: String,
    pub preco: f64,
}