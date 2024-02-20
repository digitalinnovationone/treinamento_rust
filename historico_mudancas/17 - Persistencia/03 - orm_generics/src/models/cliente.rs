use crate::macros::model_macro::create_struct_and_metadata;

create_struct_and_metadata! {
    "clientes" => Cliente {
        id: i32, "autoincrement",
        nome: String, "varchar(100)",
        telefone: String, "varchar(15)",
    }
}