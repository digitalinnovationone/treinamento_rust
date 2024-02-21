// @generated automatically by Diesel CLI.

diesel::table! {
    clientes (id) {
        id -> Integer,
        nome -> Varchar,
        telefone -> Varchar,
    }
}
