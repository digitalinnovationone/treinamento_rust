use mysql::*;
use mysql::prelude::Queryable;
use crate::repositorios::cnn::obter_conexao;
use crate::traits::sql::Generatable;

pub fn inserir<T: Into<Params>>(entidade: &T) -> Result<()> {
    let mut conn = obter_conexao()?;
    let sql = T::generate_sql_insert();
    let params = entidade.into();
    conn.exec_drop(&sql, params)?;
    Ok(())
}


pub fn atualizar<T: Generatable>(id: i32, entidade: &T) -> Result<()> {
    let mut conn = obter_conexao()?;
    let sql = T::generate_sql_update();
    let mut params = entidade.to_params();
    params.push(id.into()); // Assumindo que `id` é sempre o último parâmetro no SQL UPDATE

    conn.exec_drop(sql, params)?;

    Ok(())
}

pub fn excluir<T: Generatable>(id: i32) -> Result<()> {
    let mut conn = obter_conexao()?;
    let sql = T::generate_sql_delete();

    conn.exec_drop(sql, params!{"id" => id})?;

    Ok(())
}

pub fn listar<T: Generatable>() -> Result<Vec<T>> {
    let mut conn = obter_conexao()?;
    let sql = T::generate_sql_select();
    let entidades = conn.query_map(sql, T::from_row)?; // `from_row` precisa ser implementada para converter uma linha do banco em uma entidade

    Ok(entidades)
}
