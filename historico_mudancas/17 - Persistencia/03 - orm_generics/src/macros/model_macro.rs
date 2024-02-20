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
