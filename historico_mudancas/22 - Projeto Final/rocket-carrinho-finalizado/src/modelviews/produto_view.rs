use serde::Serialize;

#[derive(Serialize)]
pub struct ProdutoView {
    pub id: u32,
    pub nome: String,
    pub descricao: String,
    pub imagem: String,
    pub preco: f64,
    pub quantidade: u32,
}