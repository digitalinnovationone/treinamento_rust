#[derive(FromForm)]
pub struct ClienteDto {
    pub nome: String,
    pub telefone: String,
}
