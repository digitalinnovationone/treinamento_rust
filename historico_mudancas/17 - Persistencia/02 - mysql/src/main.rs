mod config;
mod models;
mod repositorios;
mod tela;

use std::io;
use tela::display;

fn main() {
    loop {
        println!("CRUD Clientes");
        println!("1. Criar Cliente");
        println!("2. Listar Clientes");
        println!("3. Atualizar Cliente");
        println!("4. Excluir Cliente");
        println!("5. Sair");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Falha ao ler a entrada");

        match escolha.trim() {
            "1" => display::criar_cliente(),
            "2" => {
                if let Err(e) = display::mostrar_clientes() {
                    println!("Erro ao listar clientes: {}", e);
                }
            },
            "3" => {
                if let Err(e) = display::atualizar_cliente() {
                    println!("Erro ao atualizar cliente: {}", e);
                }
            },
            "4" => {
                if let Err(e) = display::excluir_cliente() {
                    println!("Erro ao excluir cliente: {}", e);
                }
            },
            "5" => {
                println!("Saindo...");
                break;
            },
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}





// mod config; 
// mod models; 
// use models::cliente::Cliente;
// use mysql::prelude::Queryable;

// fn main() -> Result<(), mysql::Error> {
//     let mut cnn = config::cnn::obter_conexao()?;

//     let clientes: Vec<Cliente> = cnn.query_map(
//         "SELECT id, nome, telefone FROM clientes",
//         |(id, nome, telefone)| {
//             Cliente { id: id, nome, telefone }
//         },
//     )?;


//     for cliente in clientes {
//         println!("----------------------------------"); // Risco na tela
//         println!("ID: {}", cliente.id);
//         println!("Nome: {}", cliente.nome);
//         println!("Telefone: {}", cliente.telefone);
//     }
//     println!("----------------------------------"); // Risco após o último cliente


//     Ok(())
// }

