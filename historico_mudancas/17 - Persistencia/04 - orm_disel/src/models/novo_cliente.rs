use crate::schema::clientes;

#[derive(Insertable)]
#[table_name="clientes"]
pub struct NovoCliente {
    pub nome: String,
    pub telefone: String,
}
