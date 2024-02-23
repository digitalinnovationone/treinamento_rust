fn exemplo_unwrap() {
    let opcao: Option<i32> = Some(5);
    if let Some(valor) = opcao {
        println!("Valor: {}", valor);
    } else {
        println!("Opção era None");
    }
}

fn main() {
    exemplo_unwrap();
}
