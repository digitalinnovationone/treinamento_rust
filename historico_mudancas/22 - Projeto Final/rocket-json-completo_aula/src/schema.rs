// @generated automatically by Diesel CLI.

diesel::table! {
    administradores (id) {
        id -> Unsigned<Integer>,
        nome -> Varchar,
        email -> Varchar,
        senha -> Varchar,
    }
}

diesel::table! {
    recursos (id) {
        id -> Unsigned<Integer>,
        titulo -> Varchar,
        descricao -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    administradores,
    recursos,
);
