#[derive(Queryable)]
// #[table_name="clientes"]
pub struct Cliente {
    pub id: i32,
    pub nome: String,
    pub telefone: String,
}
