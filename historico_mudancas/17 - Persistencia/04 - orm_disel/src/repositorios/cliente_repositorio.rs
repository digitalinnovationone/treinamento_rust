use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use crate::schema::clientes;

use crate::models::cliente::Cliente;
use crate::models::novo_cliente::NovoCliente;

pub fn criar(conn: &MysqlConnection, nome_str: &str, telefone_str: &str) -> Result<(), Error> {
    let novo_cliente = NovoCliente {
        nome: nome_str.to_string(),
        telefone: telefone_str.to_string(),
    };

    diesel::insert_into(clientes::table)
        .values(&novo_cliente)
        .execute(conn)?;

    Ok(())
}

pub fn listar(conn: &MysqlConnection) -> Result<Vec<Cliente>, Error> {
    clientes::table.load::<Cliente>(conn)
}

pub fn atualizar(conn: &MysqlConnection, id_cliente: i32, novo_nome: &str, novo_telefone: &str) -> Result<(), Error> {
    diesel::update(clientes::table.find(id_cliente))
        .set((clientes::nome.eq(novo_nome), clientes::telefone.eq(novo_telefone)))
        .execute(conn)?;

    Ok(())
}

pub fn excluir(conn: &MysqlConnection, id_cliente: i32) -> Result<(), Error> {
    diesel::delete(clientes::table.find(id_cliente))
        .execute(conn)?;

    Ok(())
}
