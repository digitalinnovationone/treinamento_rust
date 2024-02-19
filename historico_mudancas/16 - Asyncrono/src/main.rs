//// ===== Concorrência: Exemplo basico de thread ==========

// use std::thread;
// use std::time::Duration;

// fn main() {
//     // Thread para imprimir números
//     let num_thread = thread::spawn(|| {
//         for i in 1..=5 {
//             println!("{}", i);
//             thread::sleep(Duration::from_millis(500));
//         }
//     });

//     // Thread para imprimir letras
//     let letter_thread = thread::spawn(|| {
//         for letter in 'a'..='e' {
//             println!("{}", letter);
//             thread::sleep(Duration::from_millis(200));
//         }
//     });

//     // Espera as threads terminarem
//     num_thread.join().unwrap();
//     letter_thread.join().unwrap();
// }






//// ===== Concorrência: Exemplo comunicação entre thread ==========

// use std::sync::mpsc; // mpsc significa multiple producer, single consumer
// use std::thread;
// use std::time::Duration;

// fn main() {
//     // Cria um canal
//     let (tx, rx) = mpsc::channel(); // tx = envia o resultado, rx = recebe o resultado

//     // Cria uma nova thread
//     thread::spawn(move || {
//         let msg = String::from("Olá, mundo!");

//         thread::sleep(Duration::from_millis(1000));

//         // Envia uma mensagem pelo canal
//         tx.send(msg).unwrap();
//         // Note que `msg` não pode mais ser usado aqui, pois `send` toma a propriedade da mensagem
//     });

//     // Recebe a mensagem na thread principal
//     let received = rx.recv().unwrap(); // espera até a thread ser resolvida
//     println!("Mensagem recebida: {}", received);
// }



//// ===== Concorrência: Exemplo de migração de produtos ==========
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// struct Cliente {
//     id: u32,
//     nome: String,
// }

// struct Produto {
//     id: u32,
//     nome: String,
// }

// struct Pedido {
//     cliente_id: u32,
//     produto_ids: Vec<u32>,
// }

// fn main() {
//     let (tx_clientes, rx_clientes) = mpsc::channel();
//     let (tx_produtos, rx_produtos) = mpsc::channel();

//     // Thread para criar clientes
//     let tx_clientes_clone = tx_clientes.clone();
//     let cliente_thread = thread::spawn(move || {
//         let clientes = vec![
//             Cliente { id: 1, nome: "Cliente 1".into() },
//             Cliente { id: 2, nome: "Cliente 2".into() },
//         ];

//         for cliente in clientes {
//             println!("Enviando cliente {} ...", cliente.nome);
//             tx_clientes_clone.send(cliente).unwrap();
//             thread::sleep(Duration::from_millis(100));
//         }
//     });

//     // Thread para criar produtos
//     let produto_thread = thread::spawn(move || {
//         let produtos = vec![
//             Produto { id: 1, nome: "Produto 1".into() },
//             Produto { id: 2, nome: "Produto 2".into() },
//         ];

//         for produto in produtos {
//             println!("Enviando produto {} ...", produto.nome);
//             tx_produtos.send(produto).unwrap();
//             thread::sleep(Duration::from_millis(100));
//         }
//     });

//     // Espera as threads terminarem para garantir que todos os dados foram enviados
//     //=== certo
//     cliente_thread.join().unwrap();
//     produto_thread.join().unwrap();

//     //=== errado
//     // thread::sleep(Duration::from_millis(1000));


//     // Coleta clientes e produtos após as threads terem terminado
//     let clientes: Vec<Cliente> = rx_clientes.try_iter().collect();
//     let produtos: Vec<Produto> = rx_produtos.try_iter().collect();

//     // Criação de pedido
//     if !clientes.is_empty() && !produtos.is_empty() {
//         let pedido = Pedido {
//             cliente_id: clientes[0].id,
//             produto_ids: produtos.iter().map(|p| p.id).collect(),
//         };

//         println!("Pedido criado para o cliente ID: {} com os produtos IDs: {:?}", pedido.cliente_id, pedido.produto_ids);
//     } else {
//         println!("Não há clientes ou produtos suficientes para criar um pedido.");
//     }
// }


//// ==== Concorrência Segura (Ownership e Borrowing) =======

//// ==== Ilegal =======
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let mut contador = 0;
    
//     let num_thread = thread::spawn(|| {
//         for _ in 0..10 {
//             contador += 1;
//             thread::sleep(Duration::from_millis(500));
//         }
//     });

//     num_thread.join().unwrap();

//     println!("Resultado: {}", contador);
// }



//// ==== legal =======

// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     // `Arc` permite compartilhar a propriedade do mutex entre várias threads.
//     // `Mutex` garante que apenas uma thread possa acessar o valor em um determinado momento.
//     let contador = Arc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         // Clona a referência Arc para o contador para usar em uma nova thread.
//         let contador_clonado = Arc::clone(&contador);
//         let handle = thread::spawn(move || {
//             // Bloqueia o mutex para obter acesso ao valor.
//             // A chamada `lock` pode falhar, por isso é um Result que deve ser tratado com `unwrap`.
//             let mut num = contador_clonado.lock().unwrap();

//             // Acesso exclusivo ao valor dentro do mutex.
//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     // Espera todas as threads terminarem.
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Resultado: {}", *contador.lock().unwrap());
// }







//// ===== Paralelismo ==========
/// === Rayon para Paralelismo de Dados ===
// use rayon::prelude::*;

// fn main() {
//     let nums = vec![1, 2, 3, 4, 5];
    
//     // Paralelizando a operação de mapeamento com Rayon
//     let squares: Vec<_> = nums.par_iter() // Use `par_iter` para iterar em paralelo
//                                .map(|&num| num + 1)
//                                .collect();

//     println!("Soma mais 1: {:?}", squares);
// }






//// ===== Paralelismo Limitando número de threads ==========
// use rayon::prelude::*;
// use rayon::ThreadPoolBuilder;

// fn main() {
//     let num_threads = 4; // Exemplo: limitar a 4 threads -> Verificar a quantidade de Clocks do seu servidor
//     ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();


//     let nums = vec![1, 2, 3, 4, 5];
    
//     // Paralelizando a operação de mapeamento com Rayon
//     let squares: Vec<_> = nums.par_iter() // Use `par_iter` para iterar em paralelo
//                                .map(|&num| num + 1)
//                                .collect();

//     println!("Soma mais 1: {:?}", squares);
// }






//// ===== Não Paralelismo ==========
// use std::fs::File;
// use std::io::{Write, Result};
// use std::thread;
// use std::time::Duration;

// #[derive(Debug)]
// struct Cliente {
//     id: u32,
//     nome: String,
// }

// fn salvar_cliente_transformado(cliente: &Cliente) -> Result<()> {
//     let arquivo_nome = format!("cliente_{}.txt", cliente.id);
//     let mut arquivo = File::create(arquivo_nome)?;
//     // Simulação de uma transformação de dados
//     writeln!(arquivo, "ID: {}\nNome: {}", cliente.id, cliente.nome.to_uppercase())
// }

// fn main() -> Result<()> {
//     // Simulando uma lista de clientes
//     let clientes = vec![
//         Cliente { id: 1, nome: String::from("Alice") },
//         Cliente { id: 2, nome: String::from("Bob") },
//         Cliente { id: 3, nome: String::from("Carlos") },
//         Cliente { id: 4, nome: String::from("Diana") },
//         Cliente { id: 5, nome: String::from("Eduardo") },
//         Cliente { id: 6, nome: String::from("Fernanda") },
//         Cliente { id: 7, nome: String::from("Gabriel") },
//         Cliente { id: 8, nome: String::from("Helena") },
//         Cliente { id: 9, nome: String::from("Igor") },
//         Cliente { id: 10, nome: String::from("Julia") },
//         Cliente { id: 11, nome: String::from("Kevin") },
//         Cliente { id: 12, nome: String::from("Lúcia") },
//         Cliente { id: 13, nome: String::from("Mário") },
//         Cliente { id: 14, nome: String::from("Nívea") },
//         Cliente { id: 15, nome: String::from("Olívia") },
//         Cliente { id: 16, nome: String::from("Paulo") },
//         Cliente { id: 17, nome: String::from("Quintino") },
//         Cliente { id: 18, nome: String::from("Rosa") },
//         Cliente { id: 19, nome: String::from("Sérgio") },
//         Cliente { id: 20, nome: String::from("Tânia") },
//         Cliente { id: 21, nome: String::from("Úrsula") },
//         Cliente { id: 22, nome: String::from("Vítor") },
//         Cliente { id: 23, nome: String::from("Wanda") },
//         Cliente { id: 24, nome: String::from("Xuxa") },
//         Cliente { id: 25, nome: String::from("Yago") },
//         Cliente { id: 26, nome: String::from("Zara") },
//         Cliente { id: 27, nome: String::from("Amanda") },
//         Cliente { id: 28, nome: String::from("Bruno") },
//         Cliente { id: 29, nome: String::from("Cecília") },
//         Cliente { id: 30, nome: String::from("Daniel") },
//     ];


//     clientes.iter()
//             .try_for_each(|cliente| {
//                 thread::sleep(Duration::from_millis(500));
//                 salvar_cliente_transformado(cliente)
//             })?;

//     Ok(())
// }







//// ===== Paralelismo ==========
// use rayon::prelude::*;
// use std::fs::File;
// use std::io::{Write, Result};
// use std::thread;
// use std::time::Duration;
// // use rayon::ThreadPoolBuilder;

// #[derive(Debug)]
// struct Cliente {
//     id: u32,
//     nome: String,
// }

// fn salvar_cliente_transformado(cliente: &Cliente) -> Result<()> {
//     let arquivo_nome = format!("cliente_{}.txt", cliente.id);
//     let mut arquivo = File::create(arquivo_nome)?;
//     // Simulação de uma transformação de dados
//     writeln!(arquivo, "ID: {}\nNome: {}", cliente.id, cliente.nome.to_uppercase())
// }

// fn main() -> Result<()> {
//     // Simulando uma lista de clientes
//     let clientes = vec![
//         Cliente { id: 1, nome: String::from("Alice") },
//         Cliente { id: 2, nome: String::from("Bob") },
//         Cliente { id: 3, nome: String::from("Carlos") },
//         Cliente { id: 4, nome: String::from("Diana") },
//         Cliente { id: 5, nome: String::from("Eduardo") },
//         Cliente { id: 6, nome: String::from("Fernanda") },
//         Cliente { id: 7, nome: String::from("Gabriel") },
//         Cliente { id: 8, nome: String::from("Helena") },
//         Cliente { id: 9, nome: String::from("Igor") },
//         Cliente { id: 10, nome: String::from("Julia") },
//         Cliente { id: 11, nome: String::from("Kevin") },
//         Cliente { id: 12, nome: String::from("Lúcia") },
//         Cliente { id: 13, nome: String::from("Mário") },
//         Cliente { id: 14, nome: String::from("Nívea") },
//         Cliente { id: 15, nome: String::from("Olívia") },
//         Cliente { id: 16, nome: String::from("Paulo") },
//         Cliente { id: 17, nome: String::from("Quintino") },
//         Cliente { id: 18, nome: String::from("Rosa") },
//         Cliente { id: 19, nome: String::from("Sérgio") },
//         Cliente { id: 20, nome: String::from("Tânia") },
//         Cliente { id: 21, nome: String::from("Úrsula") },
//         Cliente { id: 22, nome: String::from("Vítor") },
//         Cliente { id: 23, nome: String::from("Wanda") },
//         Cliente { id: 24, nome: String::from("Xuxa") },
//         Cliente { id: 25, nome: String::from("Yago") },
//         Cliente { id: 26, nome: String::from("Zara") },
//         Cliente { id: 27, nome: String::from("Amanda") },
//         Cliente { id: 28, nome: String::from("Bruno") },
//         Cliente { id: 29, nome: String::from("Cecília") },
//         Cliente { id: 30, nome: String::from("Daniel") },
//     ];


//     // Configuração do Rayon (opcional, dependendo do caso de uso)
//     // ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();

//     clientes.par_iter()
//             .try_for_each(|cliente| {
//                 thread::sleep(Duration::from_millis(500));
//                 salvar_cliente_transformado(cliente)
//             })?;

//     Ok(())
// }







//// === Tokio para Programação Assíncrona ===

/*
Tokio é um runtime assíncrono para Rust, projetado para facilitar a escrita de 
código não bloqueante, especialmente útil para aplicações de I/O, 
como servidores web e clientes, bancos de dados e serviços de rede.
*/

///// ==== Exemplo blocante =======
// use std::net::{TcpListener};
// use std::io::{self, Read, Write};

// fn main() -> io::Result<()> {
//     // Cria um ouvinte TCP que escuta na porta 8080 do localhost
//     let listener = TcpListener::bind("127.0.0.1:8080")?;
    
//     println!("Servidor rodando em 127.0.0.1:8080");

//     // Loop para aceitar conexões e lidar com elas
//     for stream in listener.incoming() {
//         match stream {
//             Ok(mut stream) => {
//                 println!("Cliente conectado");

//                 // Lê a mensagem do cliente
//                 let mut buf = [0; 1024];
//                 let bytes_read = stream.read(&mut buf)?;

//                 // Se a leitura foi bem-sucedida e bytes foram recebidos
//                 if bytes_read > 0 {
//                     let message = String::from_utf8_lossy(&buf[..bytes_read]);
//                     println!("Mensagem recebida: {}", message);

//                     // Envia uma resposta de volta para o cliente
//                     let response = "Mensagem recebida com sucesso!\n";
//                     stream.write_all(response.as_bytes())?;
//                 }

//                 // Fecha a conexão
//                 println!("Conexão encerrada");
//             }
//             Err(e) => {
//                 eprintln!("Erro ao aceitar conexão: {}", e);
//             }
//         }
//     }

//     Ok(())
// }



///// ===== Exemplo de http blocante ======
// testar = curl localhost:8080 # Acessa e libera
// testando = telnet localhost 8080 # bloqueia a requisição
// testar = curl localhost:8080 # trava pois o telnet bloqueou

///// ==== Exemplo não blocante =======
// use tokio::net::TcpListener;
// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     // Cria um ouvinte TCP que escuta na porta 8080 do localhost
//     let listener = TcpListener::bind("127.0.0.1:8080").await?;

//     println!("Servidor rodando em 127.0.0.1:8080");

//     loop {
//         // Aceita uma conexão
//         let (mut socket, _) = listener.accept().await?;

//         println!("Cliente conectado");

//         // Usa `tokio::spawn` para lidar com a conexão em uma nova tarefa assíncrona
//         tokio::spawn(async move {
//             let mut buf = vec![0; 1024]; // Buffer para armazenar os dados recebidos

//             // Loop para ler dados do socket e escrever de volta (ecoar)
//             loop {
//                 match socket.read(&mut buf).await {
//                     // Se a leitura for bem-sucedida e alguns bytes forem recebidos
//                     Ok(n) if n == 0 => {
//                         println!("Conexão encerrada pelo cliente");
//                         return;
//                     },
//                     Ok(n) => {
//                         // Escreve os dados de volta para o socket (ecoar)
//                         if socket.write_all(&buf[..n]).await.is_err() {
//                             println!("Erro ao enviar dados de volta");
//                             return;
//                         }
//                     },
//                     Err(e) => {
//                         println!("Erro ao ler do socket: {:?}", e);
//                         return;
//                     }
//                 }
//             }
//         });
//     }
// }
///// ===== Exemplo de http não blocante ======
// testar = curl localhost:8080 # Acessa e libera
// testando = telnet localhost 8080 # bloqueia a requisição
// testar = curl localhost:8080 # roda mesmo tendo outra requisição blocada vindo pelo telnet
