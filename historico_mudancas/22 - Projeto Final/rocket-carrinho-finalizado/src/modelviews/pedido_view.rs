use serde::Serialize;
use crate::models::cliente::Cliente;
use crate::modelviews::produto_view::ProdutoView;
use chrono::{NaiveDateTime};

#[derive(Serialize)]
pub struct PedidoView {
    pub id: u32,
    pub valor_total: f64,
    pub cliente_id: u32,
    pub data: NaiveDateTime,
    pub pago: bool,
    pub cliente: Cliente,
    pub produtos: Vec<ProdutoView>,
}

impl Default for PedidoView {
    fn default() -> Self {
        PedidoView {
            id: 0,
            valor_total: 0.0,
            cliente_id: 0, 
            data: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
            pago: false,
            cliente: Cliente::default(),
            produtos: Vec::new(),
        }
    }
}
