//! # Meu Projeto
//!
//! Este é um exemplo de aplicação que demonstra várias funcionalidades em Rust.

/// Calcula a soma de dois números.
///
/// # Exemplos
///
/// ```
/// let resultado = soma(5, 3);
/// assert_eq!(resultado, 8);
/// ```
fn soma(a: i32, b: i32) -> i32 {
    a + b
}

/// Calcula a diferença entre dois números.
///
/// # Exemplos
///
/// ```
/// let resultado = diferenca(10, 4);
/// assert_eq!(resultado, 6);
/// ```
fn diferenca(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplica dois números.
///
/// # Exemplos
///
/// ```
/// let resultado = multiplica(4, 2);
/// assert_eq!(resultado, 8);
/// ```
fn multiplica(a: i32, b: i32) -> i32 {
    a * b
}


/// Função principal que roda o programa.
fn main() {
    println!("Soma de 5 e 3 é {}", soma(5, 3));
    println!("Diferença entre 10 e 4 é {}", diferenca(10, 4));
    println!("Multiplicação de 4 por 2 é {}", multiplica(4, 2));
}
