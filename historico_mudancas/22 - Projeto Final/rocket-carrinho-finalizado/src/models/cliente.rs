use serde::Serialize;

#[derive(Serialize)]
pub struct Cliente {
    pub id: u32,
    pub nome: String,
    pub telefone: String,
}

impl Default for Cliente {
    fn default() -> Self {
        Cliente {
            id: 0, // Use valores padrão sensíveis para a estrutura
            nome: String::new(),
            telefone: String::new(),
            // Inclua outros campos conforme necessário, inicializando-os com valores padrão
        }
    }
}