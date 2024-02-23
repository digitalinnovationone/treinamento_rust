//// ===== Correto =======
fn checa_valor(valor: Option<i32>) {
    if let Some(3) = valor {
        println!("Encontrado 3");
    }
}

fn main() {
    println!("Iniciando verificação...");
    checa_valor(Some(3));
}
