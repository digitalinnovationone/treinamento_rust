#[derive(FromForm)]
pub struct ClienteDto {
    pub nome: String,
    pub cpf: String,
}
