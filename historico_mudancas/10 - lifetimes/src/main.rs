/*
Em Rust, "lifetimes" (tempo de vida) são uma forma de o compilador 
garantir que referências não persistam além da existência dos dados 
aos quais elas apontam, prevenindo assim erros comuns como 
dangling pointers (ponteiros para dados já desalocados). 

Borrow Checker = "checagem de empréstimos" foi projetado para prevenir erros
*/

// fn retorna_mensagem_de_teste() -> &String {
//     let local = String::from("isso é um teste");
//     &local // Tentativa de retornar uma referência para uma String local
// }

// fn main() {
//     let result = retorna_mensagem_de_teste();
//     println!("lifetime error {}", result);
// }

// // ================ Solução ===================
// fn retorna_mensagem_de_teste() -> String {
//     let local = String::from("isso é um teste");
//     local // Retorna a String diretamente / uma transferência de propriedade (ownership)
// }

// fn main() {
//     let result = retorna_mensagem_de_teste(); // a result se torna "ownership" do valor da string
//     println!("lifetime error {}", result);
// }


// ================ Solução ===================
/*
// 'a = uma anotação de lifetime (tempo de vida)
São usados para garantir que referências a dados não outlive (sobrevivam mais que) 
os dados aos quais elas apontam. 

Eles são uma parte fundamental do sistema de tipos de Rust, 
permitindo que o compilador verifique em tempo de compilação que os 
dados referenciados não serão desalocados enquanto ainda existirem 
referências a eles, evitando assim dangling references (referências penduradas) 
e garantindo segurança de memória.
*/
// fn quem_e_maior<'a>(x: &'a str, y: &'a str) -> &'a str {

//     println!("Endereço de memória de x: {:?}", x.as_ptr());
//     println!("Endereço de memória de y: {:?}", y.as_ptr());

//     if x.chars().count() > y.chars().count() {
//         x // retornando o ganhador
//     } else {
//         y // retornando o ganhador
//     }
// }

// fn main() {
//     let string1 = String::from("abcd");
//     println!("Endereço de memória de string1: {:?}", string1.as_ptr());

//     let string2 = "xyz";
//     println!("Endereço de memória de string2: {:?}", string2.as_ptr());

//     let result = quem_e_maior(string1.as_str(), string2);
//     println!("A maior string é {}", result);
//     println!("Endereço de memória de result o ganhador: {:?}", result.as_ptr());
// }


// ================ Erro ===================

// fn quem_e_maior(x: &str, y: &str) -> &str {
//     println!("Endereço de memória de x: {:?}", x.as_ptr());
//     println!("Endereço de memória de y: {:?}", y.as_ptr());

//     if x.len() > y.len() {
//         x // retornando o ganhador
//     } else {
//         y // retornando o ganhador
//     }
// }


fn quem_e_maior<'a>(x: &'a str, y: &'a str) -> &'a str {

    println!("Endereço de memória de x: {:?}", x.as_ptr());
    println!("Endereço de memória de y: {:?}", y.as_ptr());

    if x.chars().count() > y.chars().count() {
        x // retornando o ganhador
    } else {
        y // retornando o ganhador
    }
}

fn main() {
    let string1 = String::from("abcd");
    println!("Endereço de memória de string1: {:?}", string1.as_ptr());

    let string2 = "xyz";
    println!("Endereço de memória de string2: {:?}", string2.as_ptr());

    let result = quem_e_maior(string1.as_str(), string2);
    println!("A maior string é {}", result);
    println!("Endereço de memória de result o ganhador: {:?}", result.as_ptr());
}