mod models;
mod tela;
mod config;
mod repositorios;

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

        let resultado = match escolha.trim() {
            "1" => display::criar_cliente(),
            "2" => display::mostrar_clientes(),
            "3" => display::atualizar_cliente(),
            "4" => display::excluir_cliente(),
            "5" => {
                println!("Saindo...");
                break;
            },
            _ => {
                println!("Opção inválida, tente novamente.");
                Ok(()) // Retorna Ok para manter a consistência de tipo
            },
        };

        // Tratamento genérico de erro para as operações CRUD
        if let Err(e) = resultado {
            println!("Ocorreu um erro: {}", e);
        }
    }
}
