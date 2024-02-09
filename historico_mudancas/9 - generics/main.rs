// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let int_point = Point { x: 5, y: 10 };
//     let float_point = Point { x: 1.0, y: 4.0 };
// }





// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };
//     println!("p.x = {}", p.x());
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
// fn imprimir_propriedades(item: &impl Serialize) {
// // fn imprimir_propriedades(item: &dyn Serialize) {
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



/*

=== Conclusão ====
Código 1 usa generics com trait bounds explicitamente, o que é útil para quando você quer 
clareza total sobre a genericidade e está preparado para lidar com a verbosidade.

Código 2 simplifica a assinatura da função usando impl Trait, tornando o código mais limpo e 
fácil de ler, mantendo a eficiência da monomorfização. Se fosse usado &dyn Serialize,
introduziria polimorfismo dinâmico com uma ligeira penalidade de desempenho, mas com benefícios
de flexibilidade.

*/










use std::fmt::Debug; // Import necessário para usar a trait Debug

#[derive(Debug)]
struct Cliente {
    id: u32,
    nome: String,
}

#[derive(Debug)]
struct Produto {
    id: u32,
    nome: String,
    preco: f64,
}

// Adicionando uma restrição para que o tipo associado implemente Debug
trait Lista {
    type Item: Debug; // Agora exige que Item implemente Debug
    fn obter_lista(&self) -> Vec<Self::Item>;
}

struct ServicoCliente;
struct ServicoProduto;

impl Lista for ServicoCliente {
    type Item = Cliente;

    fn obter_lista(&self) -> Vec<Self::Item> {
        vec![
            Cliente { id: 1, nome: "Cliente 1".to_string() },
            Cliente { id: 2, nome: "Cliente 2".to_string() },
        ]
    }
}

impl Lista for ServicoProduto {
    type Item = Produto;

    fn obter_lista(&self) -> Vec<Self::Item> {
        vec![
            Produto { id: 1, nome: "Produto 1".to_string(), preco: 10.0 },
            Produto { id: 2, nome: "Produto 2".to_string(), preco: 20.0 },
        ]
    }
}

// Garantindo que L::Item implementa Debug
fn imprimir_lista<L: Lista>(servico: &L)
where
    L::Item: Debug, // Restrição adicional aqui
{
    let lista = servico.obter_lista();
    for item in lista.iter() {
        println!("{:?}", item);
    }
}

fn main() {
    let servico_cliente = ServicoCliente;
    let servico_produto = ServicoProduto;

    println!("Clientes:");
    imprimir_lista(&servico_cliente);

    println!("\nProdutos:");
    imprimir_lista(&servico_produto);
}
