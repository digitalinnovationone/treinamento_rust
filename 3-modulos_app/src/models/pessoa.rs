
pub struct Pessoa {
    pub nome: String,
    pub documento: String,
}

impl Pessoa {
    pub fn new(nome: &str, documento: &str) -> Pessoa {
        Pessoa {
            nome: nome.to_string(),
            documento: documento.to_string(),
        }
    }

    pub fn show(&self){
        println!("Nome: {}", self.nome);
        println!("Documento: {}", self.documento);
    }
}
