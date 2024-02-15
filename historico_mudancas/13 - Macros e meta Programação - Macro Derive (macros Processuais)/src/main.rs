// Importe a macro de derivação `MeuTrait`
use macro_derive_traits::MeuTrait;
use macro_derive::MeuTrait;

#[derive(MeuTrait)]
struct MinhaStruct { }

fn main() {
    let minha_instancia = MinhaStruct { };

    // Agora você pode usar o método `minha_funcao` fornecido pelo trait `MeuTrait`
    println!("{}", minha_instancia.minha_funcao());
}
