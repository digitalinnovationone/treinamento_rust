use crate::config::cnn::establish_connection;
use crate::modelviews::pedido_view::PedidoView;
use crate::repositorios::{pedido_repositorio, pedido_produto_repositorio, produto_repositorio};

pub fn adicionar(cliente_id: u32, produto_id: u32) -> bool {
    let conn = establish_connection();

    let pedido = match pedido_repositorio::ativo(&conn, cliente_id) {
        Ok(Some(pedido)) => pedido,
        Ok(None) => {
            if let Err(err) = pedido_repositorio::criar(&conn, cliente_id, 0.0, chrono::Local::now().naive_local(), false) {
                eprintln!("Erro ao criar pedido: {}", err);
                return false;
            }
            return adicionar(cliente_id, produto_id);
        },
        Err(err) => {
            eprintln!("Erro ao buscar pedido ativo: {}", err);
            return false;
        }
    };

    let produto = match produto_repositorio::buscar_por_id(&conn, produto_id) {
        Ok(produto) => produto,
        Err(err) => {
            eprintln!("Erro ao buscar produto: {}", err);
            return false;
        }
    };

    if let Err(err) = pedido_produto_repositorio::cria_se_nao_existir_ou_atualiza_quantidade(&conn, pedido.id, produto.id) {
        eprintln!("Erro ao adicionar produto ao pedido: {}", err);
        return false;
    }

    true
}

pub fn ativo(cliente_id: u32) -> PedidoView {
    let conn = establish_connection();

    match pedido_repositorio::ativo_completo(&conn, cliente_id) {
        Ok(Some(pedido_view)) => pedido_view,
        Ok(None) => PedidoView::default(), // Retorna diretamente um PedidoView vazio se não houver pedido ativo
        Err(err) => {
            eprintln!("Erro ao buscar pedido ativo: {}", err);
            PedidoView::default() // Retorna PedidoView vazio também no caso de erro
        }
    }
}

pub fn remover_produto(pedido_id: u32, produto_id: u32) -> bool {
    let conn = establish_connection();
    
    match pedido_produto_repositorio::remove_quantidade_por_id(&conn, pedido_id, produto_id) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Erro ao remover produto: {}", e);
            false
        }
    }
}
