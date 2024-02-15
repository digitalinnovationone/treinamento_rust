/////// ===== Operador Option ======
// fn encontrar_divisor(numero: i32) -> Option<i32> {
//     if numero % 2 == 0 {
//         Some(2) // Um divisor encontrado
//     } else if numero % 3 == 0 {
//         Some(3) // Outro divisor encontrado
//     } else {
//         None // Nenhum divisor encontrado
//     }
// }

// fn main() {
//     let numero = 6;
//     match encontrar_divisor(numero) {
//         Some(divisor) => println!("Divisor encontrado: {}", divisor),
//         None => println!("Nenhum divisor encontrado para {}", numero),
//     }
// }

/////// ===== Operador Result ======
/*
// A assinatura da enumeração Result é algo como:
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

// fn calcular_raiz_quadrada(numero: f64) -> Result<f64, String> {
//     if numero >= 0.0 {
//         Ok(numero.sqrt()) // Retorna a raiz quadrada do número
//     } else {
//         Err(String::from("número negativo não possui raiz quadrada real")) // Retorna um erro
//     }
// }

// fn main() {
//     let numero = -4.0;
//     match calcular_raiz_quadrada(numero) {
//         Ok(raiz) => println!("A raiz quadrada de {} é {}", numero, raiz),
//         Err(e) => println!("Erro: {}", e),
//     }
// }




/////// ===== io::Error ======

// use std::fs::File;
// use std::io;
// use std::io::Read;

// fn ler_arquivo(nome: &str) -> Result<String, io::Error> {
//     let mut f = File::open(nome)?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn main() {
//     match ler_arquivo("meu_arquivo.txt") {
//         Ok(conteudo) => println!("Conteúdo do arquivo: {}", conteudo),
//         Err(e) => println!("Falha ao abrir o arquivo: {}", e),
//     }
// }






// /*
// A propagação de erros e o uso do operador ? em Rust são mecanismos que simplificam o 
// manejo de erros em operações que podem falhar. Eles permitem que você escreva código mais 
// limpo e legível, evitando a necessidade de múltiplos blocos match para tratar erros explicitamente. 
// O operador ? pode ser usado em funções que retornam Result ou Option, 
// e serve para desempacotar o valor de sucesso ou retornar o erro/None prematuramente da função.
// */

// //// ========= ? = retorna o erro para o chamador =====
// use std::fs::File;
// use std::io::{self, Read};

// // Função que tenta ler o conteúdo de um arquivo para uma String
// fn ler_conteudo_arquivo(nome_arquivo: &str) -> Result<String, io::Error> {
//     let mut f = File::open(nome_arquivo)?; // Se falhar, retorna o erro para o chamador
//     let mut conteudo = String::new();
//     f.read_to_string(&mut conteudo)?; // Se falhar, retorna o erro para o chamador
//     Ok(conteudo) // Retorna o conteúdo do arquivo em caso de sucesso
// }


// fn main() {
//     let nome_arquivo = "exemplo.txt";
//     match ler_conteudo_arquivo(nome_arquivo) {
//         Ok(conteudo) => println!("Conteúdo do arquivo:\n{}", conteudo),
//         Err(e) => println!("Erro ao ler o arquivo: {}", e),
//     }


//     // let nome_arquivo = "exemplo.txt";

//     // let conteudo = match ler_conteudo_arquivo(nome_arquivo) {
//     //     Ok(conteudo) => conteudo,
//     //     Err(erro) => {
//     //         eprintln!("Erro ao ler o arquivo {}: {}", nome_arquivo, erro);
//     //         return;
//     //     },
//     // };

//     // println!("Conteúdo do arquivo:\n{}", conteudo);

// }









///////// =========== Error Types Customizados com Trait Debug ============

// use regex::Regex;
// use std::error::Error;
// use std::fmt;


// #[derive(Debug)]
// enum ValidationError {
//     EmptyName,
//     NonUniqueName,
//     InvalidFormat,
// }

// // Implementando Display para o nosso erro customizado
// impl fmt::Display for ValidationError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// // Para que ValidationError possa ser tratado como um erro
// impl Error for ValidationError {}

// fn validar_nome(nome: &str, nomes_existentes: &[&str]) -> Result<(), Box<dyn Error>> {
//     // Validar se o nome não está vazio
//     if nome.is_empty() {
//         return Err(Box::new(ValidationError::EmptyName));
//     }

//     // Validar se o nome é único
//     if nomes_existentes.contains(&nome) {
//         return Err(Box::new(ValidationError::NonUniqueName));
//     }

//     // Validar o formato do nome com regex
//     let regex = Regex::new(r"^[a-zA-Z\s]+$").unwrap();
//     // Exemplos de Strings Válidas na REGEX
//     //     "Alice"
//     //     "Bob Smith"
//     //     "a b c"
//     //     "Z"

//     if !regex.is_match(nome) {
//         return Err(Box::new(ValidationError::InvalidFormat));
//     }

//     Ok(())
// }

// fn main() {
//     let nomes_existentes = vec!["Alice", "Bob"];
//     match validar_nome("Bob", &nomes_existentes) {
//         Ok(_) => println!("Nome válido"),
//         Err(e) => println!("Erro de validação: {}", e),
//     }

//     match validar_nome("", &nomes_existentes) {
//         Ok(_) => println!("Nome válido"),
//         Err(e) => println!("Erro de validação: {}", e),
//     }

//     match validar_nome("123 Danilo", &nomes_existentes) {
//         Ok(_) => println!("Nome válido"),
//         Err(e) => println!("Erro de validação: {}", e),
//     }

//     match validar_nome("Danilo", &nomes_existentes) {
//         Ok(_) => println!("Nome válido"),
//         Err(e) => println!("Erro de validação: {}", e),
//     }

//     match validar_nome("Alice", &nomes_existentes) {
//         Ok(_) => println!("Nome válido"),
//         Err(e) => {
//             match *e.downcast_ref::<ValidationError>().unwrap() {
//                 ValidationError::EmptyName => println!("Erro de validação: o nome não pode ser vazio"),
//                 ValidationError::NonUniqueName => {
//                     println!("Erro de validação: o nome não é único");
//                     println!("Por favor, escolha um nome diferente.");
//                 },
//                 ValidationError::InvalidFormat => println!("Erro de validação: o formato do nome é inválido"),
//             }
//         },
//     }
// }







// =========== Error Types Customizados com Trait Debug ============
use std::error::Error;


#[derive(Debug)]
enum ErroDeTransacao {
    SaldoInsuficiente { saldo_atual: f64, tentativa_saque: f64 },
    ErroDeAutenticacao,
    ErroDeRede,
}

impl std::fmt::Display for ErroDeTransacao {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErroDeTransacao::SaldoInsuficiente { saldo_atual, tentativa_saque } => {
                write!(f, "Saldo insuficiente: saldo atual R${}, tentativa de saque R${}", saldo_atual, tentativa_saque)
            },
            ErroDeTransacao::ErroDeAutenticacao => {
                write!(f, "Erro de autenticação: usuário não pode ser autenticado")
            },
            ErroDeTransacao::ErroDeRede => {
                write!(f, "Erro de rede: não foi possível conectar ao servidor")
            },
        }
    }
}

// Implementando o trait Error
impl std::error::Error for ErroDeTransacao {}

fn processar_transacao(valor: f64) -> Result<(), ErroDeTransacao> {
    let saldo = 100.0; // Supondo um saldo de conta fictício
    if valor > saldo {
        return Err(ErroDeTransacao::SaldoInsuficiente { saldo_atual: saldo, tentativa_saque: valor });
    }

    // Simulando um erro de autenticação
    let autenticado = false;
    if !autenticado {
        return Err(ErroDeTransacao::ErroDeAutenticacao);
    }

    // Simulando um erro de rede
    let falha_na_rede = false;
    if !falha_na_rede {
        return Err(ErroDeTransacao::ErroDeRede);
    }

    // Se chegarmos aqui, supomos que a transação foi bem-sucedida
    Ok(())
}

fn main() {
    match processar_transacao(200.0) {
        Ok(_) => println!("Transação processada com sucesso"),
        Err(e) => {
            println!("Falha ao processar transação: {}", e);

            // Aqui, você também pode acessar a causa raiz ou backtrace se necessário (requer 'std::error::Error')
            if let Some(source) = e.source() {
                println!("Causado por: {}", source);
            }
        },
    }
}
