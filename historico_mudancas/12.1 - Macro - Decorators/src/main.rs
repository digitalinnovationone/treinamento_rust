// // Definindo uma macro para criar a struct e armazenar metadados
// macro_rules! create_struct_and_metadata {
//     ($name:ident { $($field_name:ident: $field_type:ty, $metadata:expr),* $(,)? }) => {
//         // Definição da struct
//         struct $name {
//             $(pub $field_name: $field_type,)*
//         }
        
//         // Armazenando os metadados em uma função associada
//         impl $name {
//             pub fn metadata() -> Vec<(&'static str, &'static str, &'static str)> {
//                 vec![
//                     $( (stringify!($field_name), stringify!($field_type), $metadata), )*
//                 ]
//             }
//         }
//     };
// }

// // Usando a macro para criar a struct Cliente com metadados
// create_struct_and_metadata! {
//     Cliente {
//         id: i32, "autoincrement",
//         nome: String, "varchar(100)",
//         telefone: String, "varchar(15)",
//     }
// }


// fn main() {
//     // Acessando os metadados da struct Cliente
//     for (field, field_type, metadata) in Cliente::metadata() {
//         println!("Field: {}, Type: {}, Metadata: {}", field, field_type, metadata);
//     }
// }



/////// ==== Exemplo 2 =============
// macro_rules! create_struct_and_metadata {
//     ($name:ident { $($field_name:ident: $field_type:ty, $metadata:expr),* $(,)? }) => {
//         // Definição da struct
//         struct $name {
//             $(pub $field_name: $field_type,)*
//         }
        
//         // Armazenando os metadados e gerando SQL
//         impl $name {
//             pub fn metadata() -> Vec<(&'static str, &'static str, &'static str)> {
//                 vec![
//                     $( (stringify!($field_name), stringify!($field_type), $metadata), )*
//                 ]
//             }

//             // Nova função para gerar a query SQL de criação da tabela
//             pub fn generate_sql_create_table() -> String {
//                 let table_name = stringify!($name).to_lowercase();
//                 let columns = Self::metadata().iter().map(|(field, _type, meta)| {
//                     let sql_type = match *meta {
//                         "autoincrement" => "INT AUTO_INCREMENT PRIMARY KEY",
//                         _ => meta,
//                     };
//                     format!("{} {}", field, sql_type)
//                 }).collect::<Vec<String>>().join(",\n    ");
                
//                 format!("CREATE TABLE {} (\n    {}\n);", table_name, columns)
//             }
//         }
//     };
// }

// // Definição da struct Cliente usando a macro atualizada
// create_struct_and_metadata! {
//     Cliente {
//         id: i32, "autoincrement",
//         nome: String, "varchar(100)",
//         telefone: String, "varchar(15)",
//     }
// }

// create_struct_and_metadata! {
//     Pedido {
//         id: i32, "autoincrement",
//         cliente_id: i32, "int",
//         valor_total: f32, "float",
//     }
// }

// fn main() {
//     // Gerando o script SQL para criar a tabela `cliente`
//     let sql_create_table = Cliente::generate_sql_create_table();
//     println!("{}", sql_create_table);


//     let sql_create_table_pedito = Pedido::generate_sql_create_table();
//     println!("{}", sql_create_table_pedito);
// }




/////// ==== Exemplo 3 =============
// macro_rules! create_struct_and_metadata {
//     ($table_name:expr => $struct_name:ident { $($field_name:ident: $field_type:ty, $metadata:expr),* $(,)? }) => {
//         // Definição da struct
//         struct $struct_name {
//             $(pub $field_name: $field_type,)*
//         }
        
//         // Armazenando os metadados e gerando SQL com nome de tabela personalizado
//         impl $struct_name {
//             pub fn metadata() -> Vec<(&'static str, &'static str, &'static str)> {
//                 vec![
//                     $( (stringify!($field_name), stringify!($field_type), $metadata), )*
//                 ]
//             }

//             // Função atualizada para gerar a query SQL de criação da tabela com nome personalizado
//             pub fn generate_sql_create_table() -> String {
//                 let columns = Self::metadata().iter().map(|(field, _type, meta)| {
//                     let sql_type = match *meta {
//                         "autoincrement" => "INT AUTO_INCREMENT PRIMARY KEY",
//                         _ => meta,
//                     };
//                     format!("{} {}", field, sql_type)
//                 }).collect::<Vec<String>>().join(",\n    ");
                
//                 format!("CREATE TABLE {} (\n    {}\n);", $table_name, columns)
//             }
//         }
//     };
// }

// // Definindo a struct Cliente com nome de tabela personalizado usando a macro atualizada
// create_struct_and_metadata! {
//     "tbl_clientes" => Cliente {
//         id: i32, "autoincrement",
//         nome: String, "varchar(100)",
//         telefone: String, "varchar(15)",
//     }
// }


// fn main() {
//     // Gerando e exibindo o script SQL para criar a tabela `clientes`
//     let sql_create_table = Cliente::generate_sql_create_table();
//     println!("{}", sql_create_table);
// }



/////// ==== Exemplo 4 =============
macro_rules! create_struct_and_metadata {
    ($table_name:expr => $struct_name:ident { $($field_name:ident: $field_type:ty, $metadata:expr),* $(,)? }) => {
        struct $struct_name {
            $(pub $field_name: $field_type,)*
        }
        
        impl $struct_name {
            pub fn metadata() -> Vec<(&'static str, &'static str, &'static str)> {
                vec![
                    $( (stringify!($field_name), stringify!($field_type), $metadata), )*
                ]
            }

            pub fn generate_sql_create_table() -> String {
                let columns = Self::metadata().iter().map(|(field, _type, meta)| {
                    let sql_type = match *meta {
                        "autoincrement" => "INT AUTO_INCREMENT PRIMARY KEY",
                        _ => meta,
                    };
                    format!("{} {}", field, sql_type)
                }).collect::<Vec<String>>().join(",\n    ");
                
                format!("CREATE TABLE {} (\n    {}\n);", $table_name, columns)
            }

            pub fn generate_sql_drop_table() -> String {
                format!("DROP TABLE {};", $table_name)
            }

            pub fn generate_sql_insert() -> String {
                let fields = Self::metadata().iter().map(|(field, _, _)| *field).filter(|&field| field != "id").collect::<Vec<_>>().join(", ");
                let values = Self::metadata().iter().map(|_| "?").filter(|_| true).collect::<Vec<_>>().join(", ");
                format!("INSERT INTO {} ({}) VALUES ({});", $table_name, fields, values)
            }

            pub fn generate_sql_update() -> String {
                let updates = Self::metadata().iter().map(|(field, _, _)| format!("{} = ?", field)).filter(|update| update != "id = ?").collect::<Vec<_>>().join(", ");
                format!("UPDATE {} SET {} WHERE id = ?;", $table_name, updates)
            }

            pub fn generate_sql_delete() -> String {
                format!("DELETE FROM {} WHERE id = ?;", $table_name)
            }

            pub fn generate_sql_select() -> String {
                format!("SELECT * FROM {};", $table_name)
            }
        }
    };
}

create_struct_and_metadata! {
    "clientes" => Cliente {
        id: i32, "autoincrement",
        nome: String, "varchar(100)",
        telefone: String, "varchar(15)",
    }
}



fn main() {
    println!("SQL Create Table:\n{}\n", Cliente::generate_sql_create_table());
    println!("SQL Drop Table:\n{}\n", Cliente::generate_sql_drop_table());
    println!("SQL Insert:\n{}\n", Cliente::generate_sql_insert());
    println!("SQL Update:\n{}\n", Cliente::generate_sql_update());
    println!("SQL Delete:\n{}\n", Cliente::generate_sql_delete());
    println!("SQL Select:\n{}\n", Cliente::generate_sql_select());
}
