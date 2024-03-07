pub fn remove_sql_injection(mut strs: String) -> String {
    if strs.contains('\'') {
        strs.retain(|c| c != '\'');
    }
    strs
}
