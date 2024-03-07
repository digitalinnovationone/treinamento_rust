pub trait Generatable {
    fn to_params(&self) -> std::collections::HashMap<String, String>;
    fn from_raw(raw: &std::collections::HashMap<String, String>) -> Self;
    fn generate_sql_create_table() -> String;
    fn generate_sql_drop_table() -> String;
    fn generate_sql_insert() -> String;
    fn generate_sql_update() -> String;
    fn generate_sql_delete() -> String;
    fn generate_sql_select() -> String;
}
