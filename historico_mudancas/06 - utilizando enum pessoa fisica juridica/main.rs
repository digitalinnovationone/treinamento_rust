enum Tipo {
    Fisica,
    Juridica
}

struct Pessoa {
    id: i32,
    nome: String,
    documento: String,
    tipo: Tipo
}

fn exibe_documento(pessoa: &Pessoa) {
    match pessoa.tipo {
        Tipo::Fisica => println!("\
                ID: {}\n\
                Nome: {}\n\
                CPF: {}\n\
            ",
            pessoa.id,
            pessoa.nome,
            pessoa.documento,
        ),
        Tipo::Juridica => println!("\
                ID: {}\n\
                Nome: {}\n\
                CNPJ: {}\n\
            ",
            pessoa.id,
            pessoa.nome,
            pessoa.documento,
        ),
    }
}

fn main() {
    let pf = Pessoa { id: 1, nome: "Joao".to_string(), documento: "123.456.789-00".to_string(), tipo: Tipo::Fisica };
    let pj = Pessoa { id: 2, nome: "Casa do Pão".to_string(), documento: "00.000.000/0001-00".to_string(), tipo: Tipo::Juridica };

    // Exibindo os documentos
    exibe_documento(&pf);
    exibe_documento(&pj);
}
