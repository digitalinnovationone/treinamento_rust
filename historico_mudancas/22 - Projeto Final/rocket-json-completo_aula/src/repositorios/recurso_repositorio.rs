use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use crate::schema::recursos;
use crate::schema::recursos::dsl::*;
use crate::models::recurso::Recurso;

#[derive(Insertable)]
#[table_name="recursos"]
pub struct RecursoInsertDb {
    pub titulo: String,
    pub descricao: String,
}

pub fn criar(conn: &MysqlConnection, titulo_str: &str, descricao_str: &str) -> Result<(), Error> {
    let recurso = RecursoInsertDb {
        titulo: titulo_str.to_string(),
        descricao: descricao_str.to_string(),
    };

    diesel::insert_into(recursos::table)
        .values(&recurso)
        .execute(conn)?;

    Ok(())
}

pub fn listar(conn: &MysqlConnection) -> Result<Vec<Recurso>, Error> {
    recursos::table.load::<Recurso>(conn)
}

pub fn buscar_por_id(conn: &MysqlConnection, _id: u32) -> Result<Option<Recurso>, Error> {
    recursos
        .find(_id)
        .first(conn)
        .optional()
}

pub fn atualizar(conn: &MysqlConnection, id_cliente: u32, novo_titulo: &str, novo_descricao: &str) -> Result<(), Error> {
    diesel::update(recursos::table.find(id_cliente))
        .set((recursos::titulo.eq(novo_titulo), recursos::descricao.eq(novo_descricao)))
        .execute(conn)?;

    Ok(())
}

pub fn excluir(conn: &MysqlConnection, id_cliente: u32) -> Result<(), Error> {
    diesel::delete(recursos::table.find(id_cliente))
        .execute(conn)?;

    Ok(())
}
