
////////////////// Hash Map /////////////////////
// use std::collections::HashMap;

// // struct Metragem{
// //     largura: i32,
// //     altura: i32
// // }

// fn main(){

//     // let m = Metragem{
//     //     largura: 1,
//     //     altura: 4,
//     //     xxx: 2
//     // };

//     let mut dados = HashMap::new();

//     dados.insert(String::from("Largura"), 10);
//     dados.insert(String::from("Altura"), 50);
//     dados.insert(String::from("ssssss"), 50);


//     let valor = dados.get(&String::from("Largura"));

//     if let Some(valor) = valor {
//         println!("{}", valor);
//     } else {
//         println!("Chave não encontrada");
//     }
// }



// use std::collections::HashMap;
// fn main(){

//     let mut dados: HashMap<&str, i32> = HashMap::new();

//     dados.insert("Largura", 10);
//     dados.insert("Altura", 50);


//     let valor = dados.get("Largura");

//     if let Some(valor) = valor {
//         println!("{}", valor);
//     } else {
//         println!("Chave não encontrada");
//     }
// }


// use std::collections::HashMap;
// fn main(){

//     let mut dados: HashMap<&str, i32> = HashMap::new();

//     dados.insert("Largura", 10);
//     dados.insert("Altura", 50);


//     let valor = dados.get("Largura");

//     if let Some(valor) = valor {
//         println!("{}", valor);
//     } else {
//         println!("Chave não encontrada");
//     }

//     for (key, value) in &dados {
//         println!("{}: {}", key, value);
//     }
// }


// use std::collections::HashMap;
// fn main(){

//     let mut dados: HashMap<&str, i32> = HashMap::new();

//     dados.insert("Largura", 10);
//     dados.insert("Altura", 50);

//     dados.entry("Media").or_insert(30);
//     let media = dados.entry("Media").or_insert(25);
//     println!("{}", media);

//     println!("{:?}", dados);
// }


// use std::collections::HashMap;

// fn main() {
//     // Criando o HashMap com valores na construção (com array de tuplas)
//     let dados: HashMap<&str, i32> = [
//         ("Largura", 10),
//         ("Altura", 50),
//     ].iter().cloned().collect();

//     println!("HashMap completo: {:?}", dados);
// }


// use std::collections::HashMap;

// fn main() {
//     // Inicializando o HashMap diretamente com um array de tuplas
//     let dados = HashMap::from([
//         ("Largura", 10),
//         ("Altura", 50),
//     ]);

//     println!("HashMap completo: {:?}", dados);
// }


// use std::collections::HashMap;

// struct Pessoa {
//     nome: String,
//     idade: u32,
//     cidade: String,
// }

// fn main() {
//     let pessoa = Pessoa {
//         nome: "João".to_string(),
//         idade: 30,
//         cidade: "São Paulo".to_string(),
//     };

//     // Usando um array de tuplas e o método collect() para construir o HashMap
//     let mut propriedades_pessoa: HashMap<String, String> = [
//         ("nome".to_string(), pessoa.nome),
//         ("idade".to_string(), pessoa.idade.to_string()),
//         ("cidade".to_string(), pessoa.cidade),
//     ]
//     .iter()
//     .cloned()
//     .collect();

//     propriedades_pessoa.insert("Largura".to_string(), "xxx".to_string());


//     println!("{:?}", propriedades_pessoa);
// }


// use std::collections::HashMap;

// // Definição da struct que será armazenada no HashMap
// struct Dimensao {
//     largura: u32,
//     altura: u32,
// }

// fn main() {
//     // Criação de um HashMap vazio onde a chave é uma String e o valor é a struct Dimensao
//     let mut dimensoes = HashMap::new();

//     // Adicionando valores ao HashMap
//     dimensoes.insert(
//         String::from("quarto"),
//         Dimensao {
//             largura: 5,
//             altura: 4,
//         },
//     );

//     // Adicionando uma nova chave e valor ao HashMap após a inicialização
//     dimensoes.insert(
//         String::from("sala"),
//         Dimensao {
//             largura: 10,
//             altura: 8,
//         },
//     );

//     // Imprimindo os valores armazenados no HashMap
//     for (chave, dimensao) in &dimensoes {
//         println!(
//             "A {} tem {} metros de largura e {} metros de altura.",
//             chave, dimensao.largura, dimensao.altura
//         );
//     }
// }



// [dependencies]
// maplit = "1.0.2"

#[macro_use] extern crate maplit;

use std::collections::HashMap;

fn main() {
    let dados: HashMap<&str, i32> = hashmap! {
        "Largura" => 10,
        "Altura" => 50,
    };

    println!("HashMap completo: {:?}", dados);
}