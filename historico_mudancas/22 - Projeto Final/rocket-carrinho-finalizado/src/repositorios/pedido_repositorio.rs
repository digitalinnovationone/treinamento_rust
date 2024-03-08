use rusqlite::{params, Connection, Result};
use chrono::NaiveDateTime;
use crate::modelviews::{pedido_view::PedidoView, produto_view::ProdutoView};
use crate::models::cliente::Cliente;
use crate::models::pedido::Pedido;
use std::collections::HashMap;

// Cria um novo pedido
pub fn criar(conn: &Connection, cliente_id: u32, valor_total: f64, data: NaiveDateTime, pago: bool) -> Result<()> {
    conn.execute(
        "INSERT INTO pedidos (valor_total, cliente_id, data, pago) VALUES (?1, ?2, ?3, ?4)",
        params![valor_total, cliente_id, data, pago],
    )?;
    Ok(())
}


pub fn ativo(conn: &Connection, cliente_id: u32) -> Result<Option<Pedido>> {
    // Prepara a query SQL que busca o pedido ativo
    let mut stmt = conn.prepare(
        "SELECT id, cliente_id, valor_total, data, pago FROM pedidos WHERE cliente_id = ?1 AND pago = 0"
    )?;

    // Executa a query com o parâmetro cliente_id
    let mut pedido_iter = stmt.query_map(params![cliente_id], |row| {
        Ok(Pedido {
            id: row.get(0)?,
            cliente_id: row.get(1)?,
            valor_total: row.get(2)?,
            data: row.get(3)?,
            pago: row.get(4)?,
        })
    })?;

    // O método query_map retorna um iterador, então usamos next() para obter o primeiro (e único) resultado
    // que satisfaz as condições de nossa query
    let pedido_ativo = pedido_iter.next().transpose()?; // transforma Result<Option<T>, E> em Option<Result<T, E>> e então aplica transpose para Result<Option<T>, E>

    Ok(pedido_ativo)
}


// Retorna o pedido ativo para um cliente específico
pub fn ativo_completo(conn: &Connection, cliente_id: u32) -> Result<Option<PedidoView>> {
    let mut stmt = conn.prepare("
        SELECT 
            p.id AS pedido_id, p.valor_total AS pedido_valor_total, p.data AS pedido_data, p.pago AS pedido_pago,
            c.id AS cliente_id, c.nome AS cliente_nome, c.telefone AS cliente_telefone,
            pr.id AS produto_id, pr.nome AS produto_nome, pr.descricao AS produto_descricao, 
            pr.imagem AS produto_imagem, pr.preco AS produto_preco, pp.quantidade AS produto_quantidade
        FROM pedidos p
        INNER JOIN clientes c ON p.cliente_id = c.id
        INNER JOIN pedido_produtos pp ON p.id = pp.pedido_id
        INNER JOIN produtos pr ON pp.produto_id = pr.id
        WHERE p.cliente_id = ?1 AND p.pago = 0")?;

    let mut rows = stmt.query(params![cliente_id])?;

    let mut produtos_por_pedido: HashMap<u32, PedidoView> = HashMap::new();

    while let Some(row) = rows.next()? {
        let pedido_id: u32 = row.get("pedido_id")?;
        let produto_id: u32 = row.get("produto_id")?;

        let pedido_view = produtos_por_pedido.entry(pedido_id).or_insert_with(|| PedidoView {
            id: pedido_id,
            valor_total: row.get("pedido_valor_total").unwrap_or(0.0),
            cliente_id,
            data: row.get("pedido_data").unwrap(),
            pago: row.get("pedido_pago").unwrap(),
            cliente: Cliente {
                id: cliente_id,
                nome: row.get("cliente_nome").unwrap(),
                telefone: row.get("cliente_telefone").unwrap(),
            },
            produtos: Vec::new(),
        });

        pedido_view.produtos.push(ProdutoView {
            id: produto_id,
            nome: row.get("produto_nome").unwrap(),
            descricao: row.get("produto_descricao").unwrap(),
            imagem: row.get("produto_imagem").unwrap(),
            preco: row.get("produto_preco").unwrap(),
            quantidade: row.get("produto_quantidade").unwrap(),
        });
    }

    Ok(produtos_por_pedido.into_values().next())
}
