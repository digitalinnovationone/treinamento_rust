use crate::repositorios::generico_repositorio;
use crate::models::cliente::Cliente;
use std::io;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn criar_cliente() {
    limpar_tela();
    let mut nome = String::new();
    let mut telefone = String::new();

    println!("Nome do Cliente: ");
    io::stdin().read_line(&mut nome).expect("Falha ao ler o nome");

    println!("Telefone do Cliente: ");
    io::stdin().read_line(&mut telefone).expect("Falha ao ler o telefone");

    let cliente = Cliente {
        id: 0,
        nome: nome.trim().to_string(),
        telefone: telefone.trim().to_string(),
    };

    generico_repositorio::inserir(&cliente).expect("Falha ao criar cliente");

    println!("Cadastro realizado com sucesso");
    pausar_por_segundos(2);
    limpar_tela();
}

pub fn mostrar_clientes() -> Result<(), Box<dyn std::error::Error>> {
    limpar_tela();

    let clientes: Vec<Cliente> = generico_repositorio::listar()?;

    for cliente in clientes {
        println!("----------------------------------"); // Risco na tela
        println!("ID: {}", cliente.id);
        println!("Nome: {}", cliente.nome);
        println!("Telefone: {}", cliente.telefone);
    }
    println!("----------------------------------"); // Risco após o último cliente

    pausar_ate_enter();
    limpar_tela();

    Ok(())
}

fn pausar_ate_enter() {
    println!("Pressione Enter para continuar...");
    let mut _descartar = String::new(); // Variável temporária, prefixada com _ para indicar que é intencionalmente não utilizada.
    io::stdin().read_line(&mut _descartar).expect("Falha ao ler a entrada");
    limpar_tela();
}

fn limpar_tela() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "cls"])
                .status()
                .unwrap();
    } else {
        Command::new("clear")
                .status()
                .unwrap();
    }
}

fn pausar_por_segundos(segundos: u64) {
    thread::sleep(Duration::from_secs(segundos));
}

pub fn atualizar_cliente() -> Result<(), Box<dyn std::error::Error>> {
    limpar_tela();
    let mut id = String::new();
    let mut nome = String::new();
    let mut telefone = String::new();

    println!("ID do Cliente a ser atualizado: ");
    io::stdin().read_line(&mut id).expect("Falha ao ler o ID");
    let id = id.trim().parse::<i32>().expect("ID inválido");

    println!("Novo nome do Cliente: ");
    io::stdin().read_line(&mut nome).expect("Falha ao ler o nome");

    println!("Novo telefone do Cliente: ");
    io::stdin().read_line(&mut telefone).expect("Falha ao ler o telefone");

    let cliente = Cliente {
        id: id,
        nome: nome.trim().to_string(),
        telefone: telefone.trim().to_string(),
    };

    generico_repositorio::atualizar(id, &cliente).expect("Falha ao atualizar cliente");

    println!("Cliente atualizado com sucesso.");
    pausar_por_segundos(2);
    limpar_tela();

    Ok(())
}

pub fn excluir_cliente() -> Result<(), Box<dyn std::error::Error>> {
    limpar_tela();
    let mut id = String::new();

    println!("ID do Cliente a ser excluído: ");
    io::stdin().read_line(&mut id).expect("Falha ao ler o ID");
    let id = id.trim().parse::<i32>().expect("ID inválido");

    generico_repositorio::excluir::<Cliente>(id).expect("Falha ao excluir cliente");

    println!("Cliente excluído com sucesso.");
    pausar_por_segundos(2);
    limpar_tela();

    Ok(())
}
