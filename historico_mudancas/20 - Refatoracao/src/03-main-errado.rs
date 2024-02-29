fn exemplo_unwrap() {
    let opcao: Option<i32> = Some(5);
    let valor = opcao.unwrap(); // Potencial ponto de falha se for None
    println!("Valor: {}", valor);
}

fn main() {
    exemplo_unwrap();
}
