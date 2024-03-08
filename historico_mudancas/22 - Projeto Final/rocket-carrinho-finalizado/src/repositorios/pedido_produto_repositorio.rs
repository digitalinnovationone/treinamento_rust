use rusqlite::{params, Connection, Result};


pub fn cria_se_nao_existir_ou_atualiza_quantidade(conn: &Connection, pedido_id: u32, produto_id: u32) -> Result<()> {
    // Verifica se o pedido_produto já existe
    let query = "SELECT quantidade FROM pedido_produtos WHERE pedido_id = ?1 AND produto_id = ?2";
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(params![pedido_id, produto_id])?;

    let mut produto_existe = false;
    let quantidade: i32 = if let Some(row) = rows.next()? {
        produto_existe = true;
        row.get(0)?
    } else {
        0
    };

    if produto_existe {
        // Se o pedido_produto já existe, atualiza a quantidade
        let nova_quantidade = quantidade + 1;
        conn.execute(
            "UPDATE pedido_produtos SET quantidade = ?1 WHERE pedido_id = ?2 AND produto_id = ?3",
            params![nova_quantidade, pedido_id, produto_id],
        )?;
    } else {
        // Se o pedido_produto não existe, cria um novo com quantidade 1
        conn.execute(
            "INSERT INTO pedido_produtos (pedido_id, produto_id, quantidade) VALUES (?1, ?2, 1)",
            params![pedido_id, produto_id],
        )?;
    }

    // Atualiza o valor total do pedido
    let total_query = "SELECT SUM(p.preco * pp.quantidade) FROM produtos p INNER JOIN pedido_produtos pp ON p.id = pp.produto_id WHERE pp.pedido_id = ?1";
    let total: f64 = conn.query_row(total_query, params![pedido_id], |row| row.get(0))?;

    conn.execute(
        "UPDATE pedidos SET valor_total = ?1 WHERE id = ?2",
        params![total, pedido_id],
    )?;

    Ok(())
}

pub fn remove_quantidade_por_id(conn: &Connection, pedido_id: u32, produto_id: u32) -> Result<()> {
    // Verifica se o pedido_produto existe
    let query = "SELECT quantidade FROM pedido_produtos WHERE pedido_id = ?1 AND produto_id = ?2";
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(params![pedido_id, produto_id])?;

    if let Some(row) = rows.next()? {
        let quantidade: i32 = row.get(0)?;

        if quantidade > 1 {
            // Se a quantidade for maior que 1, decrementa a quantidade
            let nova_quantidade = quantidade - 1;
            conn.execute(
                "UPDATE pedido_produtos SET quantidade = ?1 WHERE pedido_id = ?2 AND produto_id = ?3",
                params![nova_quantidade, pedido_id, produto_id],
            )?;
        } else {
            // Se a quantidade for 1, exclui o registro
            conn.execute(
                "DELETE FROM pedido_produtos WHERE pedido_id = ?1 AND produto_id = ?2",
                params![pedido_id, produto_id],
            )?;
        }

        // Atualiza o valor total do pedido
        let total_query = "SELECT SUM(p.preco * pp.quantidade) FROM produtos p INNER JOIN pedido_produtos pp ON p.id = pp.produto_id WHERE pp.pedido_id = ?1";
        let total: f64 = conn.query_row(total_query, params![pedido_id], |row| row.get(0))?;

        conn.execute(
            "UPDATE pedidos SET valor_total = ?1 WHERE id = ?2",
            params![total, pedido_id],
        )?;
    }

    Ok(())
}
