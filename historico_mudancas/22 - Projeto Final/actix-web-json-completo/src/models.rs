use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Message {
    pub mensagem: String,
}

create_struct_and_metadata! {
    "clientes" => Cliente {
        id: i32, "autoincrement",
        nome: String, "varchar(100)",
        cpf: String, "varchar(15)",
    }
}

create_struct_and_metadata! {
    "administradores" => Administrador {
        id: i32, "autoincrement",
        nome: String, "varchar(100)",
        email: String, "varchar(255)",
        senha: String, "varchar(100)",
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub senha: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenApi {
    pub token: String,
}
