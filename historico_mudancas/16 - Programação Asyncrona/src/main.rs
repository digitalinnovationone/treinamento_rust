//// ==== programação Assincrona - async_std ========
/*
Programação Assíncrona
Modelo de Execução: Na programação assíncrona, uma única thread pode executar múltiplas tarefas quase simultaneamente.

Threads
Modelo de Execução: Threads permitem a execução de múltiplas tarefas em paralelo, literalmente ao mesmo tempo, em sistemas com múltiplos cores de processador
*/

// use async_std::task;

// async fn say_hello(item: String) -> String {
//     println!("Hello, async world! - {}", item);
//     format!("{} - ret - ", item)
// }

// fn main() {
//     let item = task::block_on(say_hello("01".to_string())); // o await
//     task::block_on(say_hello(item));

//     //say_hello("03"); não roda pois não em o await
// }







//// ==== programação Assincrona ========
// use async_std::task;

// async fn say_hello(item: String) -> String {
//     println!("Hello, async world! - {}", item);
//     format!("{} - ret - ", item)
// }

// async fn run() -> String {
//     let item = say_hello("01".to_string()).await;
//     let item2 = say_hello(item).await;
//     item2
// }

// fn main() {
//     let item = task::block_on(run());
//     println!("Item - {}", item);
// }




//// ==== programação Assincrona smol runtime ========
// O smol é um runtime assíncrono pequeno e eficiente para Rust, projetado para ser leve e fácil de usar. 

// fn main() {
//     smol::block_on(async {
//         println!("Hello, smol world!");
//     });
// }



//// ==== programação Assincrona smol + surf runtime ========
// async fn fetch_url(url: &str) -> surf::Result<String> {
//     let mut res = surf::get(url).await?;
//     println!("Status: {} - {}", res.status(), url);
//     // Aqui precisamos aguardar o resultado do body_string, pois é uma operação assíncrona.
//     let body = res.body_string().await?;
//     Ok(body) // Retornamos o corpo da resposta como uma String.
// }

// fn main() {
//     // Agora vamos também tratar o caso de sucesso do fetch_url, não apenas o erro.
//     match smol::block_on(fetch_url("https://google.com")) {
//         Ok(body) => println!("Corpo da resposta do Google.com: {}", body),
//         Err(e) => eprintln!("Erro ao buscar https://google.com: {}", e),
//     }

//     match smol::block_on(fetch_url("https://httpbin.org/get")) {
//         Ok(body) => println!("Corpo da resposta do httpbin.org/get: {}", body),
//         Err(e) => eprintln!("Erro ao buscar https://httpbin.org/get: {}", e),
//     }
// }



//// ==== programação Assincrona read_file ========

// use async_std::fs::File;
// use async_std::prelude::*;
// use async_std::task;

// async fn read_file() -> std::io::Result<()> {
//     let mut file = File::open("danilo.txt").await?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).await?;
//     println!("File contents: {}", contents);
//     Ok(())
// }

// fn main() {

//     match task::block_on(read_file()) {
//         Ok(_) => println!("Arquivo lido"),
//         Err(e) => eprintln!("erro ao ler arquivo: {}", e),
//     }
    
// }









//// ==== programação Assincrona ========
// use async_std::task;
// use std::time::Duration;

// async fn task_one() -> i32 {
//     task::sleep(Duration::from_millis(500)).await;
//     println!("task_one");
//     10
// }

// async fn task_two() -> i32 {
//     task::sleep(Duration::from_millis(200)).await;
//     println!("task_two");
//     20
// }

// fn main() {
//     smol::block_on(async {
//         let t1 = smol::spawn(task_one());
//         let t2 = smol::spawn(task_two());

//         let result_one = t1.await;
//         let result_two = t2.await;

//         println!("Results: {}, {}", result_one, result_two);
//     });
// }








//// ==== programação Assincrona ========
use async_std::task;
use futures::join;
use std::time::Duration;


async fn task_one() -> i32 {
    task::sleep(Duration::from_millis(500)).await;
    println!("task_one");
    10
}

async fn task_two() -> i32 {
    task::sleep(Duration::from_millis(200)).await;
    println!("task_two");
    20
}

fn main() {
    task::block_on(async {
        // Usando `join!` diretamente conforme sugerido pela mensagem de erro do compilador
        let (result_one, result_two) = join!(task_one(), task_two());
        println!("Results: {}, {}", result_one, result_two);
    });
}

