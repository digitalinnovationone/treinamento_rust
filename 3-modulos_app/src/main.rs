#[path = "./models/pessoa.rs"]
mod pessoa;

use pessoa::Pessoa;

fn main() {
    let daniel = Pessoa::new(
        "Daniel", 
        "222.222.222-33",
    );

    daniel.show();

    println!("{}", "-".to_string().repeat(20)); // Imprime uma linha divisória

    let c_e_c = Pessoa::new(
        "C&C", 
        "222.222.00000-33",
    );

    c_e_c.show();

    println!("{}", "-".to_string().repeat(20)); // Imprime uma linha divisória
}
