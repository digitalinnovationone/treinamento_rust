extern crate rusqlite;

#[derive(Debug)]
pub struct Cliente {
    pub id: i32,
    pub nome: String,
    pub telefone: String,
}
