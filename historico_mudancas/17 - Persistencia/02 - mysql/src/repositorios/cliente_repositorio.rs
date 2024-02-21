use mysql::*;
use mysql::prelude::Queryable;
use crate::models::cliente::Cliente;
use crate::config::cnn::obter_conexao;

pub fn criar(nome: &str, telefone: &str) -> Result<()> {
    let mut conn = obter_conexao()?;

    conn.exec_drop(
        r"INSERT INTO clientes (nome, telefone) VALUES (:nome, :telefone)",
        params! {
            "nome" => nome,
            "telefone" => telefone,
        },
    )?;

    Ok(())
}

pub fn listar() -> Result<Vec<Cliente>> {
    let mut conn = obter_conexao()?;
    let clientes = conn.query_map(
        "SELECT id, nome, telefone FROM clientes",
        |(id, nome, telefone)| {
            Cliente { id: id, nome, telefone }
        },
    )?;

    Ok(clientes)
}

pub fn atualizar(id: i32, novo_nome: &str, novo_telefone: &str) -> Result<()> {
    let mut conn = obter_conexao()?;

    conn.exec_drop(
        r"UPDATE clientes SET nome=:nome, telefone=:telefone WHERE id=:id",
        params! {
            "id" => id,
            "nome" => novo_nome,
            "telefone" => novo_telefone,
        },
    )?;

    Ok(())
}

pub fn excluir(id: i32) -> Result<()> {
    let mut conn = obter_conexao()?;

    conn.exec_drop(
        "DELETE FROM clientes WHERE id=:id",
        params! {
            "id" => id,
        },
    )?;

    Ok(())
}
