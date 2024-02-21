pub mod traits;

#[macro_export]
macro_rules! create_struct_and_metadata {
    ($table_name:expr => $struct_name:ident { $($field_name:ident: $field_type:ty, $metadata:expr),* $(,)? }) => {
        
        pub struct $struct_name {
            $(pub $field_name: $field_type,)*
        }
        
        impl $struct_name {
            pub fn metadata() -> Vec<(&'static str, &'static str, &'static str)> {
                vec![
                    $( (stringify!($field_name), stringify!($field_type), $metadata), )*
                ]
            }
        }

        impl model_macro::traits::sql::Generatable for $struct_name {
            fn from_raw(raw: &std::collections::HashMap<String, String>) -> Self {
                Self {
                    $(
                        $field_name: match raw.get(stringify!($field_name)) {
                            Some(value) => value.parse().unwrap_or_default(),
                            None => Default::default(),
                        },
                    )*
                }
            }

            fn to_params(&self) -> std::collections::HashMap<String, String> {
                let mut params = std::collections::HashMap::new();
                $(
                    params.insert(stringify!($field_name).to_string(), format!("{:?}", self.$field_name));
                )*
                params
            }

            fn generate_sql_create_table() -> String {
                let columns = Self::metadata().iter().map(|(field, _type, meta)| {
                    let sql_type = match *meta {
                        "autoincrement" => "INT AUTO_INCREMENT PRIMARY KEY",
                        _ => meta,
                    };
                    format!("{} {}", field, sql_type)
                }).collect::<Vec<String>>().join(",\n    ");
                
                format!("CREATE TABLE {} (\n    {}\n);", $table_name, columns)
            }

            fn generate_sql_drop_table() -> String {
                format!("DROP TABLE {};", $table_name)
            }

            fn generate_sql_insert() -> String {
                let fields = Self::metadata().iter().map(|(field, _, _)| *field).collect::<Vec<_>>().join(", ");
                let values = Self::metadata().iter().map(|(field, _, _)| format!(":{} ", field)).collect::<Vec<_>>().join(", ");
                format!("INSERT INTO {} ({}) VALUES ({});", $table_name, fields, values)
            }

            fn generate_sql_update() -> String {
                let updates = Self::metadata().iter().map(|(field, _, _)| format!("{} = :{}", field, field)).filter(|update| !update.starts_with("id =")).collect::<Vec<_>>().join(", ");
                format!("UPDATE {} SET {} WHERE id = :id;", $table_name, updates)
            }

            fn generate_sql_delete() -> String {
                format!("DELETE FROM {} WHERE id = :id;", $table_name)
            }

            fn generate_sql_select() -> String {
                format!("SELECT * FROM {};", $table_name)
            }
        }
    };
}
