// // Importe a macro de derivação `MeuTrait`
// use macro_derive_traits::MeuTrait;
// use macro_derive::MeuTrait;

// #[derive(MeuTrait)]
// struct MinhaStruct {}

// fn main() {
//     let minha_instancia = MinhaStruct {};

//     println!("{}", minha_instancia.minha_funcao());
// }


// Assumindo que MeuTrait e a macro derive(MeuTrait) estão definidos como acima

use macro_derive_traits::MeuTrait;
use macro_derive::MeuTrait;

#[derive(MeuTrait)]
struct TipoPadrao {}

// #[derive(MeuTrait)]
struct TipoCustomizado {}

// impl MeuTrait for TipoCustomizado {}

impl MeuTrait for TipoCustomizado {
    fn minha_funcao(&self) -> String {
        String::from("Implementação *** customizada de MeuTrait")
    }
}


fn imprime_mensagem<T: MeuTrait>(item: &T) {
    println!("{}", item.minha_funcao());
}

fn main() {
    let padrao = TipoPadrao {};
    let customizado = TipoCustomizado {};

    imprime_mensagem(&padrao); // Imprime: Implementação padrão de MeuTrait
    imprime_mensagem(&customizado); // Imprime: Implementação customizada de MeuTrait
}
