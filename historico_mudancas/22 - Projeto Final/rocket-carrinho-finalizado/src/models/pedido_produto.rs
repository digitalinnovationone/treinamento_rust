use serde::Serialize;

#[derive(Serialize)]
pub struct PedidoProduto {
    pub id: u32,
    pub pedido_id: u32,
    pub produto_id: u32,
    pub quantidade: i32,
}