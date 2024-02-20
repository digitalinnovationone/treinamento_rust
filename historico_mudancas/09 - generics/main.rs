// //==== função duplicada por objetivo ====
// fn contar_posicoes_inteiros(array: &[i32]) -> usize {
//     array.len()
// }

// fn contar_posicoes_floats(array: &[f64]) -> usize {
//     array.len()
// }

// fn contar_posicoes_strings(array: &[&str]) -> usize {
//     array.len()
// }

// fn main() {
//     let array_inteiros: [i32; 5] = [1, 2, 3, 4, 5];
//     let array_floats: [f64; 4] = [1.1, 2.2, 3.3, 4.4];
//     let array_strings: [&str; 3] = ["um", "dois", "três"];

//     println!("Posições no array de inteiros: {}", contar_posicoes_inteiros(&array_inteiros));
//     println!("Posições no array de floats: {}", contar_posicoes_floats(&array_floats));
//     println!("Posições no array de strings: {}", contar_posicoes_strings(&array_strings));
// }




// //==== Resolução do generics ====
// fn contar_posicoes<T>(array: &[T]) -> usize {
//     array.len()
// }

// fn main() {
//     let array_inteiros: [i32; 5] = [1, 2, 3, 4, 5];
//     let array_floats: [f64; 4] = [1.1, 2.2, 3.3, 4.4];
//     let array_strings: [&str; 3] = ["um", "dois", "três"];

//     println!("Posições no array de inteiros: {}", contar_posicoes(&array_inteiros));
//     println!("Posições no array de floats: {}", contar_posicoes(&array_floats));
//     println!("Posições no array de strings: {}", contar_posicoes(&array_strings));
// }


// //==== função duplicada por objetivo ====
// fn quantidade_digitos_inteiro(i: i32) -> usize {
//     i.to_string().chars().count()
// }

// fn quantidade_digitos_float(f: f64) -> usize {
//     f.to_string().chars().count()
// }

// fn quantidade_caracteres_string(s: &str) -> usize {
//     s.chars().count()
// }

// fn main() {
//     let int_val: i32 = 12345;
//     let float_val: f64 = 123.45;
//     let string_val: &str = "Olá josé";

//     println!("Quantidade de dígitos no inteiro: {}", quantidade_digitos_inteiro(int_val));
//     println!("Quantidade de dígitos no float: {}", quantidade_digitos_float(float_val));
//     println!("Quantidade de caracteres na string: {}", quantidade_caracteres_string(string_val));
// }


// = função duplicada por objetivo ====
// fn quantidade_digitos<T>(i: T) -> usize {
//     i.to_string().chars().count() ??????
// }

// fn main() {
//     let int_val: i32 = 12345;
//     let float_val: f64 = 123.45;
//     let string_val: &str = "Olá josé";

//     println!("Quantidade de dígitos no inteiro: {}", quantidade_digitos(int_val));
//     println!("Quantidade de dígitos no float: {}", quantidade_digitos(float_val));
//     println!("Quantidade de caracteres na string: {}", quantidade_digitos(string_val));
// }

// //==== Resolução do generics ====

// trait ContaCaracteres {
//     fn conta_caracteres(&self) -> usize;
// }

// impl ContaCaracteres for i32 {
//     fn conta_caracteres(&self) -> usize {
//         self.to_string().chars().count()
//     }
// }

// impl ContaCaracteres for f64 {
//     fn conta_caracteres(&self) -> usize {
//         self.to_string().chars().count()
//     }
// }

// impl ContaCaracteres for String {
//     fn conta_caracteres(&self) -> usize {
//         self.chars().count()
//     }
// }

// impl<'a> ContaCaracteres for &'a str {
//     fn conta_caracteres(&self) -> usize {
//         self.chars().count()
//     }
// }

// fn quantidade_caracteres<T: ContaCaracteres>(valor: T) -> usize {
//     valor.conta_caracteres()
// }

// struct Blaba {
//     x: i32
// }

// impl ContaCaracteres for Blaba {
//     fn conta_caracteres(&self) -> usize {
//         self.x.to_string().chars().count()
//     }
// }

// fn main() {

//     let int_val: i32 = 12345;
//     let float_val: f64 = 123.45;
//     let str_val: &str  = "Olá josé";
//     let string_val: String  = "Olá josé".to_string();
//     let blaba_val: Blaba = Blaba { x: 5 };

//     println!("Quantidade de caracteres no inteiro: {}", quantidade_caracteres(int_val));
//     println!("Quantidade de caracteres no float: {}", quantidade_caracteres(float_val));
//     println!("Quantidade de caracteres na string: {}", quantidade_caracteres(str_val));
//     println!("Quantidade de caracteres na string: {}", quantidade_caracteres(string_val));
//     println!("Quantidade de caracteres na blaba: {}", quantidade_caracteres(blaba_val));
// }


//==== Resolução do generics ====

/*
O trait Display da biblioteca padrão pode ser utilizado para converter 
os tipos em uma forma que possa ser representada como uma string.
Uma vez que um tipo implemente Display, ele pode ser convertido em String e, 
em seguida, podemos contar os caracteres.
*/

// use std::fmt::Display; // Trait que tem uma função comum to_string()

// fn quantidade_caracteres<T: Display>(valor: T) -> usize {
//     valor.to_string().chars().count()
// }

// // struct Blaba {
// //     x: i32
// // }

// // impl Display for Blaba {
// //     fn to_string(&self) -> usize {
// //         self.x.to_string().chars().count()
// //     }
// // }

// fn main() {
//     let int_val: i32 = 12345;
//     let float_val: f64 = 123.45;
//     let string_val: &str = "Olá josé";
//     // let blaba_val: Blaba = Blaba { x: 5 };

//     println!("Quantidade de dígitos no inteiro: {}", quantidade_caracteres(int_val));
//     println!("Quantidade de dígitos no float: {}", quantidade_caracteres(float_val));
//     println!("Quantidade de caracteres na string: {}", quantidade_caracteres(&string_val));
//     // println!("Quantidade de caracteres na blaba: {}", quantidade_caracteres(&blaba_val));
// }


// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let int_point = Point { x: 5, y: 10 };
//     let float_point = Point { x: 1.0, y: 4.0 };
//     let string_point = Point { x: "1.0", y: "4.0" };
// }





// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn retorna_valor_de_x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };
//     println!("p.x = {}", p.retorna_valor_de_x());
// }


// use std::fmt::Display;

// fn print<T: Display>(item: T) {
//     println!("{}", item);
// }

// fn main() {
//     print(1); // Int
//     print(String::from("hello")); // String
//     print("hello"); // &str
//     print(1.5); // &f64
// }





// struct Pair<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Pair<T, U> {
//     fn new(x: T, y: U) -> Self {
//         Self { x, y }
//     }
// }

// fn main() {
//     let pair = Pair::new(5, 10.5);
//     println!("{}", pair.x);

//     let pair2 = Pair::new(5, "O valor de Y");
//     println!("{}", pair2.y);
// }





/*
=== Tipos de Traits ==

===[ use std::fmt::Debug; ]===
A trait Debug é usada para habilitar a funcionalidade de formatação de saída de debug para tipos. 
Quando um tipo implementa a trait Debug, ele pode ser formatado usando o especificador 
de formatação {:?} ou {:#?} (para uma saída mais "bonita", também conhecida como "pretty print").
 Isso é particularmente útil durante o desenvolvimento e para debugging, pois permite imprimir 
 valores de uma forma legível para o desenvolvedor.

===[ use std::cmp::PartialOrd; ]===
A trait PartialOrd permite comparações parciais entre valores de um tipo, o que significa que 
nem todos os valores podem ser comparáveis entre si. Ela fornece a funcionalidade para 
verificar se um valor é menor que, igual a, ou maior que outro valor, 
retornando Some(true), Some(false), ou None quando a comparação não é possível 
(por exemplo, quando comparando números flutuantes NaN). A trait PartialOrd é uma 
supertrait da trait PartialEq, que fornece funcionalidade para testar igualdade e desigualdade.

===[ use Copy; ]===
A trait Copy em Rust é uma trait especial que indica que os valores do 
tipo em questão podem ser duplicados simplesmente copiando seus bits, uma operação 
conhecida como shallow copy. Isso é diferente de um deep copy, que copiaria não apenas 
o valor em si, mas também qualquer dado ao qual ele se refere indiretamente. 

A trait Copy é comumente implementada por tipos simples e sem alocação na heap,
como números inteiros, pontos flutuantes, e outros tipos primitivos, assim como 
tuplas e arrays desses tipos, desde que todos os elementos também implementem Copy.
*/

// use std::fmt::Debug;
// use std::cmp::PartialOrd;

// fn compare_and_display<T, U>(a: T, b: U)
// where
//     T: PartialOrd + Debug,
//     U: Into<T>,
// {
//     let b: T = b.into();
//     if a > b {
//         println!("{:?} is greater than {:?}", a, b);
//     } else {
//         println!("{:?} is not greater than {:?}", a, b);
//     }
// }

// fn main() {
//     compare_and_display(10, 5);
// }






// // Definição de uma função genérica `largest`, que encontra o maior elemento em uma lista.
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     // Usando a função `largest` com um vetor de números inteiros.
//     let numbers = vec![34, 50, 25, 100, 65];
//     println!("O maior número é {}", largest(&numbers));

//     // Usando a mesma função `largest` com um vetor de números de ponto flutuante.
//     let float_numbers = vec![34.0, 50.1, 25.4, 100.75, 65.22];
//     println!("O maior número é {}", largest(&float_numbers));
// }





// trait DatabaseService {
//     fn save_message(&self, message: &str);
//     fn show_message(&self) -> String;
// }

// struct MySQLService;

// impl DatabaseService for MySQLService {
//     fn save_message(&self, message: &str) {
//         println!("Saving '{}' to MySQL", message);
//         // Aqui iria a lógica para salvar a mensagem no MySQL
//     }

//     fn show_message(&self) -> String {
//         let message = "Message from MySQL";
//         println!("Fetching message from MySQL: {}", message);
//         // Aqui iria a lógica para buscar a mensagem do MySQL
//         message.to_string()
//     }
// }

// struct PostgreSQLService;

// impl DatabaseService for PostgreSQLService {
//     fn save_message(&self, message: &str) {
//         println!("Saving '{}' to PostgreSQL", message);
//         // Aqui iria a lógica para salvar a mensagem no PostgreSQL
//     }

//     fn show_message(&self) -> String {
//         let message = "Message from PostgreSQL";
//         println!("Fetching message from PostgreSQL: {}", message);
//         // Aqui iria a lógica para buscar a mensagem do PostgreSQL
//         message.to_string()
//     }
// }


// struct GenericService<T: DatabaseService> {
//     database_service: T,
// }

// impl<T: DatabaseService> GenericService<T> {
//     fn new(database_service: T) -> Self {
//         GenericService { database_service }
//     }

//     fn save_message(&self, message: &str) {
//         self.database_service.save_message(message);
//     }

//     fn show_message(&self) -> String {
//         self.database_service.show_message()
//     }
// }

// fn main() {
//     let mysql_service = MySQLService;
//     let postgres_service = PostgreSQLService;

//     let mysql_generic_service = GenericService::new(mysql_service);
//     let postgres_generic_service = GenericService::new(postgres_service);

//     mysql_generic_service.save_message("Hello, World!");
//     let message_from_mysql = mysql_generic_service.show_message();
//     println!("{}", message_from_mysql);

//     postgres_generic_service.save_message("Hello, Rust!");
//     let message_from_postgres = postgres_generic_service.show_message();
//     println!("{}", message_from_postgres);
// }





// use serde::Serialize;
// use serde_json::to_string_pretty;

// #[derive(Serialize)]
// struct Produto {
//     id: u32,
//     nome: String,
//     preco: f64,
// }

// #[derive(Serialize)]
// struct Cliente {
//     id: u32,
//     nome: String,
//     email: String,
// }

// /// Função genérica para imprimir propriedades e valores de uma struct
// fn imprimir_propriedades<T: Serialize>(item: &T) {
//     let json = to_string_pretty(item).unwrap_or_else(|_| "Falha na serialização".to_string());
//     println!("{}", json);
// }

// fn main() {
//     let produto = Produto {
//         id: 1,
//         nome: "Caneta".to_string(),
//         preco: 1.50,
//     };

//     let cliente = Cliente {
//         id: 101,
//         nome: "João Silva".to_string(),
//         email: "joao.silva@example.com".to_string(),
//     };

//     imprimir_propriedades(&produto);
//     imprimir_propriedades(&cliente);
// }





use serde::Serialize;
use serde_json::to_string_pretty;

#[derive(Serialize)]
struct Produto {
    id: u32,
    nome: String,
    preco: f64,
}

#[derive(Serialize)]
struct Cliente {
    id: u32,
    nome: String,
    email: String,
}

/// Função genérica para imprimir propriedades e valores de uma struct
fn imprimir_propriedades(item: &impl Serialize) {
// fn imprimir_propriedades(item: &dyn Serialize) {
    let json = to_string_pretty(item).unwrap_or_else(|_| "Falha na serialização".to_string());
    println!("{}", json);
}

fn main() {
    let produto = Produto {
        id: 1,
        nome: "Caneta".to_string(),
        preco: 1.50,
    };

    let cliente = Cliente {
        id: 101,
        nome: "João Silva".to_string(),
        email: "joao.silva@example.com".to_string(),
    };

    imprimir_propriedades(&produto);
    imprimir_propriedades(&cliente);
}



/*

=== Conclusão ====
Código 1 usa generics com trait bounds explicitamente, o que é útil para quando você quer 
clareza total sobre a genericidade e está preparado para lidar com a verbosidade.

Código 2 simplifica a assinatura da função usando impl Trait, tornando o código mais limpo e 
fácil de ler, mantendo a eficiência da monomorfização. Se fosse usado &dyn Serialize,
introduziria polimorfismo dinâmico com uma ligeira penalidade de desempenho, mas com benefícios
de flexibilidade.

*/










// use std::fmt::Debug; // Import necessário para usar a trait Debug

// #[derive(Debug)]
// struct Cliente {
//     id: u32,
//     nome: String,
// }

// #[derive(Debug)]
// struct Produto {
//     id: u32,
//     nome: String,
//     preco: f64,
// }

// // Adicionando uma restrição para que o tipo associado implemente Debug
// trait Lista {
//     type Item: Debug; // Agora exige que Item implemente Debug
//     fn obter_lista(&self) -> Vec<Self::Item>;
// }

// struct ServicoCliente;
// struct ServicoProduto;

// impl Lista for ServicoCliente {
//     type Item = Cliente;

//     fn obter_lista(&self) -> Vec<Self::Item> {
//         vec![
//             Cliente { id: 1, nome: "Cliente 1".to_string() },
//             Cliente { id: 2, nome: "Cliente 2".to_string() },
//         ]
//     }
// }

// impl Lista for ServicoProduto {
//     type Item = Produto;

//     fn obter_lista(&self) -> Vec<Self::Item> {
//         vec![
//             Produto { id: 1, nome: "Produto 1".to_string(), preco: 10.0 },
//             Produto { id: 2, nome: "Produto 2".to_string(), preco: 20.0 },
//         ]
//     }
// }

// // Garantindo que L::Item implementa Debug
// fn imprimir_lista<L: Lista>(servico: &L)
// where
//     L::Item: Debug, // Restrição adicional aqui
// {
//     let lista = servico.obter_lista();
//     for item in lista.iter() {
//         println!("{:?}", item);
//     }
// }

// fn main() {
//     let servico_cliente = ServicoCliente;
//     let servico_produto = ServicoProduto;

//     println!("Clientes:");
//     imprimir_lista(&servico_cliente);

//     println!("\nProdutos:");
//     imprimir_lista(&servico_produto);
// }
