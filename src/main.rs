fn main() {
    let x: String = String::from("Depurando código");
    let x_modificada = mostra_na_tela_alterando(x);
    print!("========================\n");
    println!("Oláaaaa !!! - {}", x_modificada);
    print!("========================\n");
}

fn mostra_na_tela_alterando(mut str: String) -> String {
    str += " - alterando ...";
    str
}
