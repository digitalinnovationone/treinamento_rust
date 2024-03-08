use serde::Serialize;
use chrono::{NaiveDateTime};

#[derive(Serialize)]
pub struct Pedido {
    pub id: u32,
    pub valor_total: f64,
    pub cliente_id: u32,
    pub data: NaiveDateTime,
    pub pago: bool,
}
