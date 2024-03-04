use crate::models::recurso::Recurso;
use crate::dtos::recurso_dto::RecursoDto;

pub fn lista_de_recursos() -> Vec<Recurso> {
    // buscando dados do banco de dados
    return vec![
        Recurso{ id: 1, titulo: "Recurso 1".to_string(), descricao: "Descricao do recurso 1".to_string() },
        Recurso{ id: 2, titulo: "Recurso 2".to_string(), descricao: "Descricao do recurso 2".to_string() },
    ];
}

pub fn busca_por_id(id:u32) -> Recurso {
    // buscando dados do banco de dados pelo ID
    println!("{}", id);

    Recurso{ id: 1, titulo: "Recurso 1".to_string(), descricao: "Descricao do recurso 1".to_string() }
}

pub fn apagar_recurso_por_id(id: u32) -> Result<(), String> {
    // usar repositório para apagar no DB
    println!("Id: {}", id);

    if true {
        Ok(())
    } else {
        Err("Erro ao excluir".to_string())
    }
}

pub fn cadastrar_recurso(recurso_dto: RecursoDto) -> Result<Recurso, String> {
    // https://chat.openai.com/share/e3fd8231-acf0-4a60-9a7b-440f87a56f2a

    // usar repositório para gravar no DB
    println!("Titulo: {}", recurso_dto.titulo);
    println!("Descricao: {}", recurso_dto.descricao);

    if true {
        Ok(Recurso { id: 1, titulo: recurso_dto.titulo, descricao: recurso_dto.descricao })
    } else {
        Err("Erro ao cadastrar".to_string())
    }
}

pub fn alterar_recurso(id:u32, recurso_dto: RecursoDto) -> Result<Recurso, String> {
    // usar repositório para gravar no DB
    println!("Id: {}", id);
    println!("Titulo: {}", recurso_dto.titulo);
    println!("Descricao: {}", recurso_dto.descricao);

    if recurso_dto.titulo == ""{
        return Err("O tiulo não pode ser vazio".to_string())
    }

    if true {
        Ok(Recurso { id: 1, titulo: recurso_dto.titulo, descricao: recurso_dto.descricao })
    } else {
        Err("Erro ao atualizar".to_string())
    }
}