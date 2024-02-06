////////////////// Enum /////////////////////
// enum Tipo {
//     Juridica,
//     Fisica,
// }

// struct Pessoa {
//     nome: String,
//     documento: String,
//     tipo: Tipo,
// }

// fn main() {
//     let daniel = Pessoa {
//         nome: String::from("Daniel"),
//         documento: String::from("99.999.999/9999-99"),
//         tipo: Tipo::Juridica,
//     };

//     // Pattern Matching
//     match daniel.tipo {
//         Tipo::Fisica => {
//             println!("{} é uma pessoa física", daniel.nome)
//         },
//         _ => {
//             println!("{} é uma pessoa jurídica", daniel.nome)
//         },
//     }
// }




#[derive(PartialEq)]
enum Tipo {
    Juridica,
    Fisica,
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
        tipo: Tipo::Fisica,
    };

    if daniel.tipo == Tipo::Fisica {
        println!("{} é uma pessoa física", daniel.nome);
    } else {
        println!("{} é uma pessoa jurídica", daniel.nome);
    }
}

