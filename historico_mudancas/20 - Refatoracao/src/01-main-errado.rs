// ===== Errado =======
fn checa_valor(valor: Option<i32>) {
    match valor {
        Some(3) => println!("Encontrado 3"),
        _ => (),
    }
}

fn main() {
    println!("Iniciando verificação...");
    checa_valor(Some(3));
}