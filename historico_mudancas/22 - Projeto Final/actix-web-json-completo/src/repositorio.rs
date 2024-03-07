use mysql::Result;
use mysql::params;
use mysql::prelude::*;
use crate::config::obter_conexao;
use model_macro::traits::sql::Generatable;
use mysql::Value;
use std::collections::HashMap;
use mysql::{Error as MySQLError};

pub fn criar_tabela<T: Generatable>() -> Result<()> {
    let mut conn = obter_conexao()?;
    let sql = T::generate_sql_create_table();
    conn.exec_drop(&sql, ())?;
    Ok(())
}

pub fn apagar_tabela<T: Generatable>() -> Result<()> {
    let mut conn = obter_conexao()?;
    let sql = T::generate_sql_drop_table();
    conn.exec_drop(&sql, ())?;
    Ok(())
}

pub fn inserir<T: Generatable>(entidade: &T) -> Result<()> {
    let mut conn = obter_conexao()?;
    let sql = T::generate_sql_insert();
    let params_map = entidade.to_params(); // Supondo que to_params retorna HashMap<String, String>
    let params = params_map.iter().map(|(k, v)| (k.as_str(), Value::from(v.clone()))).collect::<Vec<_>>();
    conn.exec_drop(&sql, params)?;
    Ok(())
}

pub fn atualizar<T: Generatable>(id: i32, entidade: &T) -> Result<()> {
    let mut conn = obter_conexao()?;
    let sql = T::generate_sql_update();
    let mut params_map = entidade.to_params();
    params_map.insert("id".to_string(), id.to_string());
    let params = params_map.iter().map(|(k, v)| (k.as_str(), Value::from(v.clone()))).collect::<Vec<_>>();
    conn.exec_drop(&sql, params)?;
    Ok(())
}

pub fn excluir<T: Generatable>(id: i32) -> Result<()> {
    let mut conn = obter_conexao()?;
    let sql = T::generate_sql_delete();
    let params = params!{
        "id" => id,
    };
    conn.exec_drop(&sql, params)?;
    Ok(())
}


pub fn listar<T: Generatable>() -> Result<Vec<T>, MySQLError> {
    let mut conn = obter_conexao()?;
    let sql = T::generate_sql_select();
    let mut entities = Vec::new();

    let query_results = conn.query_iter(sql)?;
    for row_result in query_results {
        let row = row_result?;
        let mut raw_data = HashMap::<String, String>::new();

        for (column, value) in row.columns().iter().zip(row.unwrap().iter()) {
            let column_name = column.name_str().to_string();
            let val_str = match value {
                mysql::Value::Bytes(bytes) => String::from_utf8_lossy(bytes).into_owned(),
                _ => String::from("Unsupported value type"),
            };
            raw_data.insert(column_name, val_str);
        }

        let entity = T::from_raw(&raw_data);
        entities.push(entity);
    }

    Ok(entities)
}

pub fn filtrar<T: Generatable>(where_sql:String) -> Result<Vec<T>, MySQLError> {
    let mut conn = obter_conexao()?;
    let mut sql = T::generate_sql_select();

    if let Some(';') = sql.chars().last() {
        sql.pop();
    }
    
    sql.push_str(&where_sql);

    sql.push(';');
    
    let mut entities = Vec::new();

    let query_results = conn.query_iter(sql)?;
    for row_result in query_results {
        let row = row_result?;
        let mut raw_data = HashMap::<String, String>::new();

        for (column, value) in row.columns().iter().zip(row.unwrap().iter()) {
            let column_name = column.name_str().to_string();
            let val_str = match value {
                mysql::Value::Bytes(bytes) => String::from_utf8_lossy(bytes).into_owned(),
                _ => String::from("Unsupported value type"),
            };
            raw_data.insert(column_name, val_str);
        }

        let entity = T::from_raw(&raw_data);
        entities.push(entity);
    }

    Ok(entities)
}
