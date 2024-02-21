use serde::Serialize; // Importação correta de Serialize

#[derive(Serialize)] // Garante que Cliente possa ser serializado
pub struct Cliente {
    pub id: u32,
    pub nome: String,
    pub cpf: String,
}