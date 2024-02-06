use crate::enums::tipo::Tipo;

pub struct Pessoa {
    pub nome: String,
    pub documento: String,
    pub tipo: Tipo
}

impl Pessoa {
    pub fn new(nome: &str, documento: &str, tipo: Tipo) -> Pessoa {
        Pessoa {
            nome: nome.to_string(),
            documento: documento.to_string(),
            tipo: tipo
        }
    }

    pub fn show(&self){
        println!("Nome: {}", self.nome);
        println!("Documento: {}", self.documento);
        println!("Tipo: {}", self.tipo_as_string());
    }

    fn tipo_as_string(&self) -> &str {
        match self.tipo {
            Tipo::Fisica => "Fisica",
            Tipo::Juridica => "Juridica",
        }
    }
}
