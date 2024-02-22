mod valida_cpf;

use std::io;
use valida_cpf::valida_cpf;

fn main() {
    println!("Digite o CPF para validação:");
    let mut cpf = String::new();
    io::stdin().read_line(&mut cpf).expect("Falha ao ler entrada");
    let cpf = cpf.trim(); // Remove espaços em branco e nova linha

    if valida_cpf(cpf) {
        println!("CPF válido.");
    } else {
        println!("CPF inválido.");
    }
}
