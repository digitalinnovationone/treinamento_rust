use crate::config::cnn::establish_connection;
use crate::models::produto::Produto;
use crate::repositorios::produto_repositorio;

pub fn listar() -> Vec<Produto> {
    let conn = establish_connection();
    match produto_repositorio::listar(&conn) {
        Ok(produtos) => produtos,
        Err(err) => {
            eprintln!("Erro ao listar produtos: {}", err);
            Vec::new()
        }
    }
}
