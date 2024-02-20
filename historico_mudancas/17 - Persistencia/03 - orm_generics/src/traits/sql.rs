pub trait Generatable {
    fn to_params(&self) -> Vec<Value>;
    fn from_row(row: Row) -> Self;
}
