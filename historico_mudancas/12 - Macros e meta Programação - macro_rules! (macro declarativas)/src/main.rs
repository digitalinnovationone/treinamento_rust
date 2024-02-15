
/*
macros são uma forma de metaprogramação que permite escrever código que gera outro código. 
Isso é especialmente útil para reduzir a repetição de código e aumentar a flexibilidade. 
As macros declarativas, definidas com macro_rules!, são um tipo comum de macro em Rust, 
utilizadas para definir padrões de código reutilizáveis.
*/


/////// ======= Macro declarativa simples =======

// macro_rules! diz_ola {
//     () => {
//         println!("Olá, mundo!")
//     };
// }

// fn main() {
//     diz_ola!()
// }





/////// ======= Macro declarativa com trait =======

// // Definindo um trait com métodos que queremos forçar a implementação
// trait ExemploTrait {
//     fn metodo_exemplo(&self);
// }

// // Criando uma macro para implementar automaticamente o trait para qualquer struct
// macro_rules! implementa_trait {
//     ($t:ty) => {
//         impl ExemploTrait for $t {
//             fn metodo_exemplo(&self) {
//                 println!("Método exemplo chamado para {:?}", self);
//             }
//         }
//     };
// }

// // Definindo uma struct
// #[derive(Debug)]
// struct MinhaStruct;

// // Usando a macro para aplicar a implementação do trait à struct
// implementa_trait!(MinhaStruct);

// fn main() {
//     let minha_instancia = MinhaStruct;
//     minha_instancia.metodo_exemplo();
// }




/////// ======= Macro com metaprogramação para criar structs =======

// macro_rules! cria_structs {
//     // Caso para uma única struct sem campos
//     ($nome:ident) => {
//         #[derive(Debug)]
//         struct $nome;
//     };
//     // Caso para uma struct com um campo
//     ($nome:ident, $campo1:ident: $tipo1:ty) => {
//         struct $nome {
//             $campo1: $tipo1,
//         }
//     };
//     // Caso para uma struct com dois campos
//     ($nome:ident, $campo1:ident: $tipo1:ty, $campo2:ident: $tipo2:ty) => {
//         struct $nome {
//             $campo1: $tipo1,
//             $campo2: $tipo2,
//         }
//     };
//     // E assim por diante, você pode expandir para mais campos se necessário
// }

// fn main() {
//     // Criando uma struct sem campos
//     cria_structs!(Vazia);
    
//     // Criando uma struct com um campo
//     cria_structs!(UmCampo, campo1: u32);
    
//     // Criando uma struct com dois campos
//     cria_structs!(DoisCampos, campo1: u32, campo2: String);
    
//     // Exemplo de uso
//     let vazia = Vazia { };
//     let item = UmCampo { campo1: 10 };
//     let item2 = DoisCampos { campo1: 20, campo2: String::from("Olá") };
    
//     println!("vazia: {:?}", vazia);
//     println!("UmCampo: {}", item.campo1);
//     println!("DoisCampos: {}, {}", item2.campo1, item2.campo2);
// }





/////// ======= Macro com metaprogramação para criar metodos de uma Struct =======

// // Definindo uma macro para implementar métodos especificados em uma struct.
// macro_rules! implementa_metodos {
//     // Aceita o nome da struct seguido por uma sequência de identificadores (nomes de métodos).
//     ($struct:ident, $($metodo:ident),*) => {
//         impl $struct {
//             // Para cada identificador fornecido, gera um método que imprime uma mensagem.
//             $(
//                 fn $metodo(&self) {
//                     println!("{}::{} foi chamado", stringify!($struct), stringify!($metodo));
//                 }
//             )*
//         }
//     };
// }

// // Definindo uma struct de exemplo.
// struct ExemploStruct;

// // Usando a macro para adicionar métodos à struct `ExemploStruct`.
// implementa_metodos!(ExemploStruct, metodo_a, metodo_b, metodo_c);

// fn main() {
//     let exemplo = ExemploStruct;
    
//     // Chamando os métodos gerados pela macro.
//     exemplo.metodo_a();
//     exemplo.metodo_b();
//     exemplo.metodo_c();
// }






// macro_rules! define_metodos {
//     // Caso base da recursão: Quando não há mais elementos na tupla
//     ($struct:ident, ) => {};

//     // Caso recursivo: Processa o primeiro método e invoca a si mesma para o restante
//     ($struct:ident, $metodo:ident, $($rest:ident,)*) => {
//         impl $struct {
//             fn $metodo(&self) {
//                 println!("Método {} chamado!", stringify!($metodo));
//             }
//         }
//         // Chamada recursiva para processar os métodos restantes
//         define_metodos!($struct, $($rest,)*);
//     };
// }

// // Definindo a struct
// struct MinhaStruct;

// // Aplicando a macro para criar métodos
// define_metodos!(MinhaStruct, metodo_um, metodo_dois,);

// fn main() {
//     let minha_struct = MinhaStruct;
//     minha_struct.metodo_um();
//     minha_struct.metodo_dois();
// }



////// ====== Macro para criar atributos =======

// macro_rules! define_struct_com_atributos {
//     // A macro aceita o nome da struct seguido por uma lista de pares (nome do atributo: tipo do atributo)
//     ($nome:ident, $($campo:ident: $tipo:ty),*) => {
//         struct $nome {
//             // Gera um campo para cada par nome:tipo fornecido
//             $(
//                 $campo: $tipo,
//             )*
//         }
//     };
// }

// // Usando a macro para definir uma nova struct com atributos especificados
// define_struct_com_atributos!(
//     Pessoa,
//     nome: String,
//     idade: u8,
//     email: String
// );

// fn main() {
//     // Criando uma instância da struct Pessoa
//     let pessoa = Pessoa {
//         nome: String::from("João"),
//         idade: 30,
//         email: String::from("joao@email.com"),
//     };

//     // Exemplo de acesso aos campos
//     println!("Nome: {}", pessoa.nome);
//     println!("Idade: {}", pessoa.idade);
//     println!("Email: {}", pessoa.email);
// }






///// ========== Exemplo JSON =========

// use serde::{Serialize, Deserialize};
// use serde_json::Result;
// use std::fs;


// macro_rules! cria_struct {
//     ($nome:ident { $($campo:ident: $tipo:ty),* $(,)? }) => {
//         #[derive(Debug, Serialize, Deserialize)]
//         struct $nome {
//             $($campo: $tipo),*
//         }
//     };
// }

// // Exemplo de uso da macro para criar uma struct com base nos campos especificados
// cria_struct! {
//     Cliente {
//         id: u32,
//         nome: String,
//         cpf: String,
//     }
// }

// fn main() -> Result<()> {
//     // Lendo o arquivo JSON
//     let data = fs::read_to_string("clientes.json").expect("Falha ao ler arquivo");

//     // Deserializando o JSON para um Vec<Cliente>
//     let clientes: Vec<Cliente> = serde_json::from_str(&data)?;

//     // Iterando sobre os clientes e imprimindo seus dados
//     for cliente in clientes {
//         println!("{:?}", cliente);
//     }

//     Ok(())
// }





// ///// ========== Exemplo JSON ler campos =========
// use serde_json::{Result, Value};
// use std::collections::HashMap;
// use std::fs;

// fn main() -> Result<()> {
//     // Substitua pelo caminho do seu arquivo JSON
//     let data = fs::read_to_string("clientes.json").expect("Falha ao ler arquivo");
//     let v: Value = serde_json::from_str(&data)?;

//     // Supondo que o JSON é um array de objetos
//     if let Some(array) = v.as_array() {
//         // Pegando o primeiro objeto do array como exemplo
//         let first_item = &array[0];

//         // Criando um HashMap para simular os campos da struct dinamicamente
//         let mut campos = HashMap::new();

//         // Iterando sobre os pares chave/valor do primeiro objeto
//         if let Some(obj) = first_item.as_object() {
//             for (key, value) in obj {
//                 // Inserindo os campos e valores no HashMap
//                 campos.insert(key, value);
//             }
//         }

//         // Agora, `campos` contém os dados do primeiro item do JSON de forma dinâmica
//         println!("Campos dinâmicos do primeiro cliente: {:?}", campos);
//     }

//     Ok(())
// }






// ///// ========== Cria struct com metodo =========
// // Suprimir todos os avisos em todo o arquivo
// #![allow(warnings)]

// macro_rules! cria_struct {
//     // Captura o nome da struct e seus campos
//     ($nome_struct:ident { $($campo:ident: $tipo:ty),* $(,)? }) => {
//         // Define a struct
        
//         #[derive(Debug)]
//         struct $nome_struct {
//             $($campo: $tipo,)*
//         }

//         // Implementação para a struct
//         impl $nome_struct {
//             // Método público para criar uma nova instância da struct
//             pub fn new($($campo: $tipo),*) -> Self {
//                 Self {
//                     $($campo),*
//                 }
//             }

//             // Exemplo de um método genérico que não depende de capturar `self` na macro
//             pub fn exemplo() {
//                 println!("Este é um método de exemplo.");
//             }
//         }
//     };
// }

// // Uso da macro para definir a struct e implementar métodos
// cria_struct! {
//     Cliente {
//         id: u32,
//         nome: String,
//         cpf: String,
//     }
// }

// fn main() {
//     let cliente = Cliente::new(1, "João da Silva".to_string(), "123.456.789-00".to_string());

//     Cliente::exemplo();

//     println!("{:?}", cliente)
// }







// ///// ========== Cria struct com metodo =========
// #![allow(warnings)]

// macro_rules! cria_struct {
//     (
//         $nome_struct:ident {
//             $($campo:ident: $tipo:ty),* $(,)?
//         }
//         $(fn $nome_metodo:ident(&$nome_metodo_struct:ident $(, $param_nome:ident: $param_tipo:ty)*) -> $ret_tipo:ty $corpo_metodo:block)*
//     ) => {
//         struct $nome_struct {
//             $($campo: $tipo,)*
//         }

//         impl $nome_struct {
//             $(
//                 fn $nome_metodo(&$nome_metodo_struct $(, $param_nome: $param_tipo)*) -> $ret_tipo $corpo_metodo
//             )*
//         }
//     };
// }

// cria_struct! {
//     Cliente {
//         id: u32,
//         nome: String,
//         cpf: String,
//     }

//     // Definição de métodos
//     fn mostra_nome(&self) -> String {
//         format!("Nome: {}", self.nome)
//     }

//     fn mostra_id(&self) -> String {
//         format!("ID: {}", self.id)
//     }

//     fn mais_numero_no_id(&self, numero: u32) -> String {
//         format!("ID + Numero: {}", self.id + numero)
//     }
// }

// fn main() {
//     // Agora podemos usar a struct Cliente após sua definição
//     let cliente = Cliente {
//         id: 1,
//         nome: "João da Silva".to_string(),
//         cpf: "123.456.789-00".to_string(),
//     };

//     println!("{}", cliente.mostra_nome());
//     println!("{}", cliente.mostra_id());
//     println!("{}", cliente.mais_numero_no_id( 10 ));
// }









// ///// ========== Usando Cria struct em arquivo separado =========
// #[macro_use]
// mod cria_struct;

// cria_struct! {
//     Cliente {
//         id: u32,
//         nome: String,
//         cpf: String,
//     }

//     // Definição de métodos
//     fn mostra_nome(&self) -> String {
//         format!("Nome: {}", self.nome)
//     }

//     fn mostra_id(&self) -> String {
//         format!("ID: {}", self.id)
//     }

//     fn mais_numero_no_id(&self, numero: u32) -> String {
//         format!("ID + Numero: {}", self.id + numero)
//     }
// }

// fn main() {
//     // Agora podemos usar a struct Cliente após sua definição
//     let cliente = Cliente {
//         id: 1,
//         nome: "João da Silva".to_string(),
//         cpf: "123.456.789-00".to_string(),
//     };

//     println!("{}", cliente.mostra_nome());
//     println!("{}", cliente.mostra_id());
//     println!("{}", cliente.mais_numero_no_id( 10 ));
// }









// ///// ========== Usando Cria struct com metodo de uma crate =========

#[macro_use]
extern crate minha_macro_lib;

cria_struct_crate_lib! {
    Cliente {
        id: u32,
        nome: String,
        cpf: String,
    }

    // Definição de métodos
    fn mostra_nome(&self) -> String {
        format!("Nome: {}", self.nome)
    }

    fn mostra_id(&self) -> String {
        format!("ID: {}", self.id)
    }

    fn mais_numero_no_id(&self, numero: u32) -> String {
        format!("ID + Numero: {}", self.id + numero)
    }
}

fn main() {
    // Agora podemos usar a struct Cliente após sua definição
    let cliente = Cliente {
        id: 1,
        nome: "João da Silva".to_string(),
        cpf: "123.456.789-00".to_string(),
    };

    println!("{}", cliente.mostra_nome());
    println!("{}", cliente.mostra_id());
    println!("{}", cliente.mais_numero_no_id( 10 ));
}

