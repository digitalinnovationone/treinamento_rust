use rusqlite::{params, Connection, Result};

use crate::models::cliente::Cliente;

// Cria um novo cliente
pub fn criar(conn: &Connection, nome_str: &str, telefone_str: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO clientes (nome, telefone) VALUES (?1, ?2)",
        params![nome_str, telefone_str],
    )?;
    Ok(())
}

// Lista todos os clientes
pub fn listar(conn: &Connection) -> Result<Vec<Cliente>> {
    let mut stmt = conn.prepare("SELECT id, nome, telefone FROM clientes")?;
    let clientes_iter = stmt.query_map(params![], |row| {
        Ok(Cliente {
            id: row.get(0)?,
            nome: row.get(1)?,
            telefone: row.get(2)?,
        })
    })?;

    clientes_iter.collect()
}

// Atualiza um cliente existente
pub fn atualizar(conn: &Connection, id_cliente: i32, novo_nome: &str, novo_telefone: &str) -> Result<()> {
    conn.execute(
        "UPDATE clientes SET nome = ?1, telefone = ?2 WHERE id = ?3",
        params![novo_nome, novo_telefone, id_cliente],
    )?;
    Ok(())
}

// Exclui um cliente
pub fn excluir(conn: &Connection, id_cliente: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM clientes WHERE id = ?1",
        params![id_cliente],
    )?;
    Ok(())
}
