/*
fn main() {
    /*
    Dado que eu tenha um ano de nascimanto, e faço a subtração pelo ano atual,
    Então devo ter o valor da idade da pessoa
    */
    let nome: &str = "Danilo";

    let ano_nascimento: u16 = 1983;
    let mes_nascimento: u16 = 1;
    let dia_nascimento: u16 = 31;

    let ano_atual: u16 = 2024;
    let mes_atual: u16 = 1;
    let dia_atual: u16 = 31;

    let mut idade: u16 = ano_atual - ano_nascimento;
    if mes_nascimento > mes_atual {
        idade -= 1;
    }
    else if dia_nascimento > dia_atual {
        idade -= 1;
    }

    println!("A idade da pessoa ({}) calculada para o ano de {} é de {} anos", nome, ano_nascimento, idade);
    
}
*/


// fn main() {
//     let numero: i8 = 3;

//     let resultado: i8 = if numero > 3 { 4 } else { 6 };

//     println!("o valor é : {}", resultado);
// }



// fn main() {
//     let number: i8 = 1;

//     match number {
//         1 => println!("Um"),
//         2 => println!("Dois"),
//         3 => println!("Três"),
//         _ => println!("Outro número"),
//     }
// }

// fn main() {
//     loop {
//         println!("novamente!");
//     }
// }


// fn main() {
//     let mut x = 1;
//     while x <= 20 {

//         if x >= 10 && x <= 15 { 
//             x += 1;
//             continue; 
//         }
//         println!("novamente! {}", x);

//         //if x > 10 { break; }

//         x += 1;
//     }
// }

// fn main(){
//     for number in 1..=4 {
//         println!("Número: {}", number);
//     }
// }

// use std::io;

// fn main(){
//     println!("Digite o valor da tabuada");

//     let mut valor_tabuada: String = String::new();
//     io::stdin()
//         .read_line(&mut valor_tabuada)
//         .expect("Falha ao ler a linha");


//     let valor_tabuada: i32 = valor_tabuada.trim().parse().expect("Por favor, digite um número!");

//     for multiplicador in 1..=10 {
//         println!("{} X {} = {}", multiplicador, valor_tabuada, (multiplicador * valor_tabuada))
//     }
// }


// use std::io;

// fn main(){
//     // let boleano = true;
//     // while boleano {
//     loop {
//         println!("Digite uma das opções abaixo: ");
//         println!(r#"
//             Opção 1
//             Opção 2
//             Opção 3
//             Opção 4
//         "#);

//         // println!("Opção 1\nOpção 2\nOpção 3");

//         let mut opcao: String = String::new();
//         io::stdin()
//             .read_line(&mut opcao)
//             .expect("Falha ao ler a linha");

//         let opcao: i8 = opcao.trim().parse().expect("Por favor, digite um número!");

//         match opcao {
//             1 => println!("Você escolheu a opção Um"),
//             2 => println!("Você escolheu a opção Dois"),
//             3 => println!("Você escolheu a opção Três"),
//             4 => break,
//             _ => println!("A opção que você escolheu é inválida"),
//         }
//     }
// }

// fn main() {
//     println!("Olá, mundo!");

//     outra_funcao();
//     outra_funcao();
// }

// fn outra_funcao() {
//     println!("Outra função.");
// }

// fn main() {
//     outra_funcao(5);
//     outra_funcao(15);
// }

// fn outra_funcao(x: i8) {
//     println!("O valor de x é: {}", x);
// }

// use std::result;

// fn main() {
//     soma_entre_valores(5, 20, "Valeska");
//     soma_entre_valores(5, 5, "Michele");
// }

// fn soma_entre_valores(x: i16, y: i16, nome: &str) {
//     let resultado: i16 = x + y;
//     println!("O valor da soma de x e y é: {}, {}", resultado, nome);
// }


// fn main() {
//     let x = 5;

//     println!("O valor de x é: {}", x);

//     let y = {
//         let x = 3;
//         println!("O valor de x é: {}", x);
//         x + 1
//     };

//     println!("O valor de y é: {}", y);
// }


// fn soma_entre_valores_com_valor_padrao(x: i16) -> String {
//     let y = 5;
//     soma_entre_valores(x, y)
// }

// fn soma_entre_valores(x: i16, y: i16) -> String {
//     let r = x + y;
//     format!("o valor de {} + {} é de: {}", x, y, r)
// }

// fn main() {
//     let x: String = soma_entre_valores(5, 10);

//     let y = soma_entre_valores_com_valor_padrao(3);

//     println!("O resultado da função é: {}", x);
//     println!("O resultado da função é de y: {}", y);
// }

// use std::ptr::null;

// fn numeros_recursivos(n:i32){
//     if n >= 30 { return; }
//     println!("O valor do número é de {}", n);

//     numeros_recursivos(n + 1);
// }

// fn navega_na_web(url:String, pagina: i36){
//     let body: String = http.get(url).body;
//     if body != null(){
//         println!(body);
//         navega_na_web(url + "?pagina=" + pagina);
//     }
// }

// fn main() {
//     // navega_na_web("https://www.google.com.br", 1);
//     numeros_recursivos(0);
// }

// use std::io;

// fn soma_entre_valores(x: i16, y: i16) -> i16 {
//     x + y
// }

// fn subtracao_entre_valores(x: i16, y: i16) -> i16 {
//     x - y
// }

// fn solicita_parametros_para_calculo(soma: bool){
//     let mut x: String = String::new();
//     let mut y: String = String::new();

//     println!("Digite o primeiro valor");
//     io::stdin().read_line(&mut x).expect("Falha ao ler a linha");

//     println!("Digite o segundo valor");
//     io::stdin().read_line(&mut y).expect("Falha ao ler a linha");

//     let x: i16 = x.trim().parse().expect("Por favor, digite um número!");
//     let y: i16 = y.trim().parse().expect("Por favor, digite um número!");
    
//     let r = if soma { soma_entre_valores(x, y) } else { subtracao_entre_valores(x, y) };

//     println!("O resultado entre os valores é de: {}", r);
// }

// fn solicita_tabuada(){
//     println!("Digite o valor da tabuada");

//     let mut valor_tabuada: String = String::new();
//     io::stdin().read_line(&mut valor_tabuada).expect("Falha ao ler a linha");

//     let valor_tabuada: i32 = valor_tabuada.trim().parse().expect("Por favor, digite um número!");

//     for multiplicador in 1..=10 {
//         println!("{} X {} = {}", multiplicador, valor_tabuada, (multiplicador * valor_tabuada))
//     }
// }

// fn menu(){
//     loop {
//         println!("Digite uma das opções abaixo: ");
//         println!(r#"
//             1) Soma entre valores
//             2) Subtração entre valores
//             3) Criar a tabuada de um número
//             0) Encerrar o programa
//         "#);

//         let mut opcao: String = String::new();
//         io::stdin().read_line(&mut opcao).expect("Por favor, digite um número!");

//         let opcao: i16 = opcao.trim().parse().expect("Por favor, digite um número!");

//         match opcao {
//             1 => solicita_parametros_para_calculo(true),
//             2 => solicita_parametros_para_calculo(false),
//             3 => solicita_tabuada(),
//             0 => break,
//             _ => println!("A opção que você escolheu é inválida"),
//         }
//     }
// }

// // memória static
// static xxx: i32 = 5;

// fn main() {

//     // memória stack
//     let x: i32 = 4;

//     // memória heap
//     let s: String = String::from("Danilo Aparecido sd ds dsds ds ds  ds ds ds ");
// }

/////////////////// EXEMPLOS ////////////////////////////

// fn main() {
//     // memória stack (variáveis do tipo copy no rust)
//     let x: i32 = 4;
//     let y: i32 = x; // copia de dados 

//     println!("o valor de x é {} - Referencia {:p}", x, &x);
//     println!("o valor de y é {} - Referencia {:p}", y, &y);
// }



// fn main() {
//     // memória stack (variáveis do tipo copy no rust)
//     let x: i32 = 4; // owner
//     let y: &i32 = &x; // referencia de dados (y aponta para o mesmo local que x, porém o dono é o x, se encerrar o x encerra o y )

//     println!("o valor de x é {} - Referencia {:p}", x, &x);
//     println!("o valor de y é {} - Referencia {:p}", y, y);
// }


// fn main() {
//     let x: i32 = 4;
//     let y = &x;

//     println!("O valor de x é {}", x);
//     println!("O valor de y é {}", y);

//     // Imprimindo os endereços de memória
//     println!("Endereço de memória de x: {:p}", &x); // {:p} imprime o endereço de memória de x
//     println!("Endereço de memória de y: {:p}", y);  // y já é uma referência, então podemos usá-lo diretamente
// }


// fn main() {
//     let x: i32 = 4;
//     let y: &i32 = &x; // y é uma referência para x

//     println!("O valor de x é {}", x);
//     println!("O valor de y é {}", y);

//     // Imprimindo os endereços de memória
//     println!("Endereço de memória de x: {:p}", &x); // {:p} imprime o endereço de memória de x
//     println!("Endereço de memória de y: {:p}", y);  // y já é uma referência, então podemos usá-lo diretamente
// }


// fn main() {
//     let x: i32 = 4; // owner
//     let y: &i32 = &x; // y é uma referência para x

//     println!("O valor de x é {}", x);
//     println!("O valor de y é {}", y);

//     // Imprimindo os endereços de memória
//     println!("Endereço de memória de x: {:p}", &x); // {:p} imprime o endereço de memória de x
//     println!("Endereço de memória de y: {:p}", y);  // y já é uma referência, então podemos usá-lo diretamente

//     let t = y; // cria outra referencia para o dono x
//     println!("Endereço de memória de t: {:p}", t);

//     // let j = 8;
//     // let u = 1;
//     // let i = u * j;

//     let w = *y; // Desreferência com copy para w
//     println!("O valor de w {}, Endereço de memória de w: {:p}", w, &w);
// }


// fn main() {
//     let mut x: i32 = 4; // Declare x como mutável
//     let y: &i32 = &x;

//     println!("O valor de x é {}", x);
//     println!("O valor de y é {}", y);

//     // Modifique x para invalidar y
//     // println!("O valor de y é {}", y);
//     x = 42; // modificando o owner

//     // Agora, y se tornou uma referência inválida
//     // Tentar imprimir y resultará em um erro de tempo de compilação
//     // println!("O valor de y é {}", y);
//     println!("O valor de y é {}", x);
// }


// fn main() {
//     // memória stack (variáveis do tipo copy no rust)
//     let x: i32 = 4;
//     let y = &x; // copia de dados 

//     imprime_valor(&x);
//     imprime_valor(y);
// }

// fn imprime_valor(valor: &i32){
//     println!("Endereço de memória: {:p}", valor);
// }


// fn main() {
//     // memória stack (variáveis do tipo copy no rust)
//     let mut x: i32 = 4;

//     imprime_valor(&x);
//     imprime_valor(&x);
// }

// fn imprime_valor(valor: &i32){
//     valor += 1; // não pode porque tenho imudabilidade nas referencias
//     println!("Valor {}, Endereço de memória de y: {:p}", valor, valor);
// }

// fn main() {
//     let mut x: i32 = 4;
//     println!("[Original] - Valor de x após as modificações: {} - referencia: {:p}", x, &x);

//     imprime_valor(&mut x); // Passando uma referência mutável para x
    
//     println!("[Original] - Valor de x após as modificações: {} - referencia: {:p}", x, &x);

//     imprime_valor(&mut x); // Passando uma referência mutável para x

//     println!("[Original] - Valor de x após as modificações: {} - referencia: {:p}", x, &x);
// }

// fn imprime_valor(valor: &mut i32) {
//     *valor += 1; // Modificando o valor referenciado por valor utilizando um reborrowing
//     // O compilador pode mover a variável temporariamente para uma localização diferente na memória durante a referência mutável. 
//     // O objetivo é evitar possíveis problemas de aliasing e garantir a segurança das referências mutáveis
//     println!("[reborrowing] - Valor referenciado por valor: {} - referencia: {:p}", valor, &valor);
// }


// fn main() {
//     // variáveis na memória heap

//     // não é uma variável by copy
//     let s1: String = String::from("Olá"); // s1 possui a propriedade da String
//     let s2 = s1; // A propriedade é transferida de s1 para s2 (Borrowing)

//     // Isso causa um erro, porque s1 não é mais válido após a transferência
//     // println!("s1: {} - referencia: {:p}", s1, &s1);

//     // s2 é válido e pode ser usado
//     println!("s2: {} - referencia: {:p}", s2, &s2);
// }


// fn main() {
//     // variáveis na memória heap

//     let s1 = String::from("Olá"); // s1 possui a propriedade da String
//     let s2 = s1.clone(); // s2 recebe uma cópia profunda (clone) de s1

//     // Isso causa um erro, porque s1 não é mais válido após a transferência
//     println!("s1: {} - referencia: {:p}", s1, &s1);

//     // s2 é válido e pode ser usado
//     println!("s2: {} - referencia: {:p}", s2, &s2);
// }


// fn main() {
//     // variáveis na memória heap

//     let s1 = String::from("Olá"); // s1 possui a propriedade da String
//     let s2 = s1; // A propriedade é transferida de s1 para s2 (Borrowing)

//     // Isso causa um erro, porque s1 não é mais válido após a transferência
//     println!("Antes da transferência:");
//     // print_string(&s1);
//     print_string(&s2);
// }

// fn print_string(s: &String) {
//     println!("Valor da String: {} - referencia: {:p}", s, s);
// }


// fn main() {
//     // Exemplo com String
//     let s1 = String::from("Olá, mundo!"); // s1 é uma String alocada na heap
//     let s2 = s1.clone(); // Clonando a String s1 para criar s2

//     println!("String s1: {} - referencia: {:p}", s1, &s1);
//     println!("String s2: {} - referencia: {:p}", s2, &s2);

//     // Exemplo com &str
//     let s3 = "Olá, mundo!"; // s3 é um &str (slice de string)
//     let s4 = s3; // s4 é uma referência para o mesmo &str

//     println!("&str s3: {} - referencia: {:p}", s3, s3);
//     println!("&str s4: {} - referencia: {:p}", s4, s4);
// }



// fn main() {
//     // Exemplo com String
//     let mut s1 = String::from("Olá, mundo!"); // s1 é uma String alocada na heap
//     s1 += " - teste";

//     let s2 = s1.clone(); // Clonando a String s1 para criar s2

//     println!("String s1: {} - referencia: {:p}", s1, &s1);
//     println!("String s2: {} - referencia: {:p}", s2, &s2);

//     // Exemplo com &str
//     let s3 = "Olá, mundo!"; // s3 é um &str (slice de string)
//     s3 += " - teste";

//     let s4 = s3; // s4 é uma referência para o mesmo &str

//     println!("&str s3: {} - referencia: {:p}", s3, s3);
//     println!("&str s4: {} - referencia: {:p}", s4, s4);
// }


// fn main() {
//     // Exemplo com String
//     let mut s1 = String::from("Olá, mundo!"); // s1 é uma String alocada na heap
//     s1 += " - teste";

//     let s2 = s1.clone(); // Clonando a String s1 para criar s2

//     println!("String s1: {} - referencia: {:p}", s1, &s1);
//     println!("String s2: {} - referencia: {:p}", s2, &s2);

//     // Exemplo com &str
//     let s3 = "Olá, mundo!"; // s3 é um &str (slice de string)

//     let s4 = format!("{} - teste", s3); // Criando um novo &str concatenando s3 e " - teste"

//     // let s5 = s4; // altera a posse

//     println!("&str s3: {} - referencia: {:p}", s3, s3);
//     println!("&str s4: {} - referencia: {:p}", s4, &s4);
// }


// fn main() {
//     let original_string = String::from("Rust é incrível");
    
//     // Criando uma substring usando slicing
//     let substring = &original_string[0..4]; // Obtendo os primeiros 4 caracteres
    
//     println!("String original: {} - referencia: {:p}", original_string, &original_string);
//     println!("Substring: {} - referencia: {:p}", substring, substring);
// }


// fn main() {
//     // Convertendo String em &str usando as_str()
//     let s1 = String::from("Olá, mundo!");
//     let reference_to_s1: &str = s1.as_str();
    
//     println!("s1 (String): {} - referencia: {:p}", s1, &s1);
//     println!("s1 (referência &str): {} - referencia: {:p}", reference_to_s1, reference_to_s1);
    
//     // Convertendo String em &str fazendo uma referência
//     let s2 = String::from("Rust é incrível!");
//     let reference_to_s2: &str = &s2;

//     println!("s2 (String): {} - referencia: {:p}", s2, &s2);
//     println!("s2 (referência &str): {} - referencia: {:p}", reference_to_s2, reference_to_s2);
// }



// fn main() {
//     // Convertendo String em &str usando as_str()
//     let s1: &str = "Olá, mundo!";
//     let reference_to_s1: String = format!("{}", s1);
    
//     println!("s1 (referência &str): {} - referencia: {:p}", s1, s1);
//     println!("reference_to_s1 (String): {} - referencia: {:p}", reference_to_s1, &reference_to_s1);
// }


// fn main(){
//     let mut s = String::from("olá");

//     s = s + ", mundo!"; // push_str() adiciona um literal à String
//     // s += ", mundo!"; // push_str() adiciona um literal à String
//     // s.push_str(", mundo!"); // push_str() adiciona um literal à String

//     println!("{}", s); // Isso vai exibir `olá, mundo!`
// }


// fn main() {
//     let mut x = 5;
//     manda_referencia(&mut x);
// }

// fn manda_referencia(x: &mut i32){
//     *x += 1;
//     println!("{}", x)
// }

// fn main() {
//     let mut x: i32 = 5;
//     x = manda_via_copia(x);
    
//     println!("{}", x)
// }

// fn manda_via_copia(x: i32) -> i32{
//     x + 1
// }

// fn main() {
//     let s1 = String::from("texto");

//     let tamanho = calcula_tamanho(&s1);

//     println!("O tamanho de '{}' é {}.", s1, tamanho);
// }

// fn calcula_tamanho(s: &String) -> usize {
//     s.len()
// }


// fn main(){
//     for i in 1..5 {
//         println!("{}", i);
//     }
// }

// fn main() {
//     // Criando uma tupla com três elementos de tipos diferentes
//     let tupla: (i32, f64, u8) = (500, 6.4, 66);

//     // Acessando os elementos da tupla
//     let (x, y, z) = tupla;

//     println!("O valor de x é: {}", x);
//     println!("O valor de y é: {}", y);
//     println!("O valor de z é: {}", z);

//     // Acessando diretamente os elementos da tupla
//     println!("O primeiro valor é: {}", tupla.0);
//     println!("O segundo valor é: {}", tupla.1);
//     println!("O terceiro valor é: {}", tupla.2);
// }


// fn calcular_dimensoes() -> (i32, i32) {
//     // Suponha que esses valores foram calculados
//     let largura = 30;
//     let altura = 50;
//     (largura, altura) // Retornando uma tupla
// }

// fn main() {
//     let dimensoes = calcular_dimensoes();
//     println!("Largura: {}, Altura: {}", dimensoes.0, dimensoes.1);

//     let (largura, altura) = calcular_dimensoes();
//     println!("Largura: {}, Altura: {}", largura, altura);
// }

// fn main() {
//     let tupla_aninhada: ((i32, i32), String) = ((5, 10), String::from("Rust"));

//     // Acessando elementos da tupla aninhada
//     println!("Número: ({}, {})", (tupla_aninhada.0).0, (tupla_aninhada.0).1);
//     println!("String: {}", tupla_aninhada.1);
// }

// fn main() {
//     // Declara um array mutável de 3 inteiros.
//     let mut valores: [i32; 3] = [10, 20, 30];

//     // Modifica o segundo elemento do array.
//     valores[1] = 25;

//     println!("valor do indice 0: {}", valores[0]);
//     println!("quantidade de valores do array: {}", valores.len());

//     for n in valores.iter() {
//         println!("{}", n);
//     }

// }


// fn main(){
//     // Cria um vetor vazio de inteiros e adiciona elementos a ele.
//     let mut vetor: Vec<i32> = Vec::new();
//     vetor.push(10);
//     vetor.push(20);
//     vetor.push(20);
//     vetor.push(20);
//     vetor.push(21);

//     println!("quantidade de valores do array: {}", vetor.len());

//     for n in vetor.iter() {
//         println!("{}", n);
//     }

//     let valor: Option<i32> = vetor.pop();

//     if let Some(numero) = valor{
//         println!("O valor de pop: {}", numero);
//     }

//     vetor.pop();
//     vetor.pop();
//     vetor.pop();

//     println!("quantidade de valores do array: {}", vetor.len());

//     for n in vetor.iter() {
//         println!("{}", n);
//     }

// }


fn main(){
    
}