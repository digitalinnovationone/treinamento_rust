use rusqlite::{params, Connection, Result};

use crate::models::produto::Produto;

// Lista todos os produtos
pub fn listar(conn: &Connection) -> Result<Vec<Produto>> {
    let mut stmt = conn.prepare("SELECT id,  nome, descricao, imagem, preco FROM produtos")?;
    let produtos_iter = stmt.query_map(params![], |row| {
        Ok(Produto {
            id: row.get(0)?,
            nome: row.get(1)?,
            descricao: row.get(2)?,
            imagem: row.get(3)?,
            preco: row.get(4)?,
        })
    })?;

    produtos_iter.collect()
}

pub fn buscar_por_id(conn: &Connection, id: u32) -> Result<Produto> {
    conn.query_row(
        "SELECT id, nome, descricao, imagem, preco FROM produtos WHERE id = ?1",
        params![id],
        |row| {
            Ok(Produto {
                id: row.get(0)?,
                nome: row.get(1)?,
                descricao: row.get(2)?,
                imagem: row.get(3)?,
                preco: row.get(4)?,
            })
        },
    )
}