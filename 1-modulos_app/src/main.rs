mod enums;
mod models;

use enums::*;
use models::Pessoa;

fn main() {    
    let daniel = Pessoa::new(
        "Daniel", 
        "222.222.222-33",
        Tipo::Fisica
    );

    daniel.show();

    println!("{}", "-".to_string().repeat(20)); // Imprime uma linha divisória

    let c_e_c = Pessoa::new(
        "C&C", 
        "222.222.00000-33",
        Tipo::Juridica
    );

    c_e_c.show();

    println!("{}", "-".to_string().repeat(20)); // Imprime uma linha divisória

    let sexo_f = Sexo::Feminino;
    let sexo_m = Sexo::Masculino;
    let sexo_o = Sexo::Outros;

    println!("{}", sexo_string(sexo_f));
    println!("{}", sexo_string(sexo_m));
    println!("{}", sexo_string(sexo_o));
}

fn sexo_string(sexo: Sexo) -> &'static str {
    match sexo {
        Sexo::Masculino => "Masculino",
        Sexo::Feminino => "Feminino",
        Sexo::Outros => "Outros",
    }
}

