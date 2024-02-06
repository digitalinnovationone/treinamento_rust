// Exemplo de Destructuring (Desestruturação)

// struct Pessoa {
//     nome: String,
//     idade: u8,
// }

// fn main() {
//     let pessoa = Pessoa {
//         nome: String::from("João"),
//         idade: 30,
//     };

//     // Desestruturação de uma struct
//     let Pessoa { nome, idade } = pessoa;

//     println!("Nome: {}, Idade: {}", nome, idade);
// }


// fn main() {
//     let arr = [1, 2, 3, 4, 5];

//     // Desestruturação de um array
//     let [primeiro, segundo, ..] = arr;

//     println!("Primeiro: {}, Segundo: {}", primeiro, segundo);
// }


// fn main() {
//     let tupla: (&str, i32, f32) = ("João", 2021, 5.6);


//     // Desestruturação de uma tupla
//     let (linguagem, ano, ..) = tupla;

//     let charsss = linguagem.chars();
//     let quantidade = charsss.count();


//     println!("Linguagem: {}, Ano: {}", linguagem, ano);
// }


// enum Mensagem {
//     Enviar { id: u32, texto: String },
// }

// fn main() {
//     let msg = Mensagem::Enviar { id: 1, texto: String::from("Olá, Rust!") };

//     match msg {
//         Mensagem::Enviar { id, texto } => {
//             println!("ID: {}, Texto: {}", id, texto);
//         }
//     }
// }


// struct Ponto {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let ponto = Ponto { x: 10, y: 20 };

//     // Desestruturação de uma struct
//     let Ponto { x, y } = ponto;

//     println!("x: {}, y: {}", x, y);
// }



// struct Pessoa {
//     nome: String,
//     sobrenome: String,
//     idade: u8,
// }

// fn main() {
//     let pessoa = Pessoa {
//         nome: String::from("João"),
//         sobrenome: "Teste".to_string(),
//         idade: 30,
//     };

//     // Desestruturação de uma struct
//     let pessoa2 = Pessoa{ nome: "Silva", ... } = pessoa;

//     println!("Pessoa Nova: {}", pessoa2.nome);
// }


#[derive(Debug)] // Deriva o traço Debug para permitir a impressão de debug
struct Pessoa {
    nome: String,
    sobrenome: String,
    idade: u8,
}

fn main() {
    let pessoa = Pessoa {
        nome: String::from("João"),
        sobrenome: String::from("Teste"),
        idade: 30,
    };

    // Desestruturação de uma struct
    let Pessoa { nome, sobrenome, idade } = &pessoa;

    let pessoa2 = Pessoa {
        nome: nome.to_owned() + " Silva",
        sobrenome: sobrenome.to_owned(), // Faz uma cópia do valor
        idade: *idade, // (desreferenciação) não cria uma cópia do valor; em vez disso, ela apenas fornece acesso ao valor original que a referência aponta.
    };

    println!("Pessoa Nova: {:#?}", pessoa2);
}