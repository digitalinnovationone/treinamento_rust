mod validar_cnpj;
mod validar_cpf;
mod divide_zero;

use std::io;

fn main() {
    println!("Digite o CNPJ para validação:");
    let mut cnpj = String::new();
    io::stdin().read_line(&mut cnpj).expect("Falha ao ler entrada");
    let cnpj = cnpj.trim(); // Remove espaços em branco e nova linha

    if validar_cnpj::validar(cnpj) {
        println!("O CNPJ é valido")
    } else {
        println!("O CNPJ é invalido")
    }
}
