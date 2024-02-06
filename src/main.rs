// Exemplo de Pattern Matching (Combinação de Padrões)
// enum Comando {
//     Iniciar(String),
//     Parar,
// }

// fn executar_comando(comando: Comando) {
//     match comando {
//         Comando::Iniciar(mensagem) => println!("Iniciando: {}", mensagem),
//         Comando::Parar => println!("Parando"),
//     }
// }

// fn main() {
//     let comando_iniciar = Comando::Iniciar(String::from("Motor"));
//     let comando_iniciar2 = Comando::Iniciar(String::from("Combustão"));
//     let comando_iniciar3 = Comando::Iniciar(String::from("Rolagem"));
//     let comando_parar = Comando::Parar;

//     executar_comando(comando_iniciar);
//     executar_comando(comando_iniciar2);
//     executar_comando(comando_iniciar3);
//     executar_comando(comando_parar);
// }





// enum Tipo {
//     Juridica(String),
//     Fisica(String),
// }

// struct Pessoa {
//     nome: String,
//     documento: String,
//     tipo: Tipo,
// }

// fn main() {
//     let daniel = Pessoa {
//         nome: String::from("Daniel"),
//         documento: String::from("23323.3322332.323.332"),
//         tipo: Tipo::Juridica(String::from("funcionario")),
//     };

//     match daniel.tipo {
//         Tipo::Fisica(descricao) => {
//             println!("{} é uma pessoa física({})", daniel.nome, descricao)
//         },
//         Tipo::Juridica(descricao) => {
//             println!("{} é uma pessoa jurídica, {}", daniel.nome, descricao)
//         },
//     }
// }




// enum Tipo {
//     Juridica(String),
//     Fisica(i32),
// }

// struct Pessoa {
//     nome: String,
//     documento: String,
//     tipo: Tipo,
// }

// fn main() {
//     let daniel = Pessoa {
//         nome: String::from("Daniel"),
//         documento: String::from("23323.3322332.323.332"),
//         tipo: Tipo::Juridica("xxxx".to_string()),
//     };

//     match daniel.tipo {
//         Tipo::Fisica(valor) => {
//             println!("{} é uma pessoa física({})", daniel.nome, valor)
//         },
//         Tipo::Juridica(valor) => {
//             println!("{} é uma pessoa jurídica, {}", daniel.nome, valor)
//         },
//     }
// }





// enum Tipo {
//     Juridica(String, i16),
//     Fisica(i32, f64),
// }

// struct Pessoa {
//     nome: String,
//     documento: String,
//     tipo: Tipo,
// }

// fn main() {
//     let daniel = Pessoa {
//         nome: String::from("Daniel"),
//         documento: String::from("23323.3322332.323.332"),
//         tipo: Tipo::Juridica("xxdsdsds".to_string(), 5),
//     };

//     match daniel.tipo {
//         Tipo::Fisica(ref valor, ref valor_f) => {
//             println!("{} é uma pessoa física({}), {}", daniel.nome, valor, valor_f)
//         },
//         Tipo::Juridica(ref valor, ref valor_f) => {
//             println!("{} é uma pessoa jurídica, {} {}", daniel.nome, valor, valor_f)
//         },
//     }
// }


enum Tipo {
    Juridica { mascara_cnpj: String, cnpj_governo: bool },
    Fisica { mascara_cpf: String, cpf_governo: bool },
}

struct Pessoa {
    nome: String,
    documento: String,
    tipo: Tipo,
}

fn main() {
    let daniel = Pessoa {
        nome: String::from("Daniel"),
        documento: String::from("23323.3322332.323.332"),
        tipo: Tipo::Fisica {
            mascara_cpf: String::from("999.999.999-99"),
            cpf_governo: true,
        },
    };

    match daniel.tipo {
        Tipo::Juridica { mascara_cnpj, cnpj_governo } => {
            println!(
                "{} é uma pessoa jurídica, Mascara CNPJ: {} - Governo: {}",
                daniel.nome, mascara_cnpj, cnpj_governo
            );
        }
        Tipo::Fisica { mascara_cpf, cpf_governo } => {
            println!(
                "{} é uma pessoa física, Mascara CPF: {}, documento: {} - Governo: {}",
                daniel.nome, mascara_cpf, daniel.documento, cpf_governo
            );
        }
    }
}


