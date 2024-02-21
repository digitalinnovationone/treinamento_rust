use crate::models::cliente::Cliente;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek};
use uuid::Uuid;
use serde_json;

const CLIENTES_JSON: &str = "db/clientes.json";

pub fn criar(nome: &str, telefone: &str) -> std::io::Result<()> {
    let mut arquivo_leitura = File::open(CLIENTES_JSON).unwrap_or_else(|_| File::create(CLIENTES_JSON).unwrap());
    let mut dados = String::new();
    arquivo_leitura.read_to_string(&mut dados)?;
    drop(arquivo_leitura); // Encerra explicitamente o uso do arquivo de leitura antes de reabri-lo para escrita.

    let mut clientes: Vec<Cliente> = serde_json::from_str(&dados).unwrap_or_else(|_| Vec::new());

    // Adiciona o novo cliente.
    let novo_cliente = Cliente {
        id: Uuid::new_v4(),
        nome: nome.to_string(),
        telefone: telefone.to_string(),
    };
    clientes.push(novo_cliente);

    // Serializa a lista de clientes atualizada para JSON.
    let dados_json = serde_json::to_string(&clientes)?;

    // Reabre o arquivo para escrita, truncando-o automaticamente.
    let mut arquivo_escrita = OpenOptions::new().write(true).truncate(true).open(CLIENTES_JSON)?;
    arquivo_escrita.write_all(dados_json.as_bytes())?;

    Ok(())
}

pub fn listar() -> std::io::Result<Vec<Cliente>> {
    let mut arquivo = File::open(CLIENTES_JSON)?;
    
    let mut dados = String::new();
    arquivo.read_to_string(&mut dados)?;
    
    let clientes: Vec<Cliente> = serde_json::from_str(&dados)?;
    Ok(clientes)
}

pub fn atualizar(id: Uuid, novo_nome: &str, novo_telefone: &str) -> std::io::Result<()> {
    let mut arquivo = OpenOptions::new().read(true).write(true).open(CLIENTES_JSON)?;

    let mut dados = String::new();
    arquivo.read_to_string(&mut dados)?;

    let mut clientes: Vec<Cliente> = serde_json::from_str(&dados)?;

    if let Some(cliente) = clientes.iter_mut().find(|c| c.id == id) {
        cliente.nome = novo_nome.to_string();
        cliente.telefone = novo_telefone.to_string();
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Cliente não encontrado"));
    }

    let dados_json = serde_json::to_string(&clientes)?;
    arquivo.set_len(0)?;
    arquivo.seek(std::io::SeekFrom::Start(0))?; // Reposiciona o cursor para o início do arquivo antes de escrever
    arquivo.write_all(dados_json.as_bytes())?;

    Ok(())
}

pub fn excluir(id: Uuid) -> std::io::Result<()> {
    let mut arquivo = OpenOptions::new().read(true).write(true).open(CLIENTES_JSON)?;

    let mut dados = String::new();
    arquivo.read_to_string(&mut dados)?;

    let mut clientes: Vec<Cliente> = serde_json::from_str(&dados)?;

    clientes.retain(|cliente| cliente.id != id); // Remove o cliente com o ID especificado

    let dados_json = serde_json::to_string(&clientes)?;
    arquivo.set_len(0)?;
    arquivo.seek(std::io::SeekFrom::Start(0))?; // Reposiciona o cursor para o início do arquivo antes de escrever
    arquivo.write_all(dados_json.as_bytes())?;

    Ok(())
}