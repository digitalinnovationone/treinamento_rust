trait Pessoa {
    fn mostra(&self);
} 

struct PessoaFisica {
    id: i32,
    nome: String,
    cpf: String
}

impl Pessoa for PessoaFisica {
    fn mostra(&self) {
        println!("\
            ID: {}\n\
            Nome: {}\n\
            CPF: {}\n\
            ",
            self.id,
            self.nome,
            self.cpf,
        )
    }
}

struct PessoaJuridica {
    id: i32,
    nome: String,
    cnpj: String
}

impl Pessoa for PessoaJuridica {
    fn mostra(&self) {
        println!("\
            ID: {}\n\
            Nome: {}\n\
            CNPJ: {}\n\
            ",
            self.id,
            self.nome,
            self.cnpj,
        )
    }
}

fn exibe_documento(pessoa: &dyn Pessoa) {
    pessoa.mostra();
}

fn main() {
    let pf = PessoaFisica { id: 1, nome: "Joao".to_string(), cpf: "123.456.789-00".to_string() };
    let pj = PessoaJuridica { id: 2, nome: "Casa do Pão".to_string(), cnpj: "00.000.000/0001-00".to_string() };

    // Exibindo os documentos
    exibe_documento(&pf);
    exibe_documento(&pj);
}
