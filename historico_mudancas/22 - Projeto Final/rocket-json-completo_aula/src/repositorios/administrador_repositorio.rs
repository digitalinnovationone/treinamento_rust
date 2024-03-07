use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use crate::schema::administradores;
use crate::models::administrador::Administrador;
use crate::schema::administradores::dsl::*;

pub fn buscar_por_email_senha(conn: &MysqlConnection, _email: &str, _senha: &str) -> Result<Administrador, Error> {
    administradores.filter(email.eq(_email))
        .filter(senha.eq(_senha))
        .first(conn)
}