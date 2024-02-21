#[macro_use]
extern crate model_macro;

mod config;
mod models;
mod tela;
mod repositorios;

use std::io;
use tela::display;

use crate::repositorios::generico_repositorio;
use crate::models::cliente::Cliente;

fn main() {
    generico_repositorio::apagar_tabela::<Cliente>().expect("Falha ao excluir cliente");
    generico_repositorio::criar_tabela::<Cliente>().expect("Falha ao excluir cliente");

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