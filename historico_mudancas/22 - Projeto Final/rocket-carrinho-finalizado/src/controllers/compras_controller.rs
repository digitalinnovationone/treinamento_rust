use rocket_dyn_templates::{Template, context};
use crate::servicos::produto_servico;
use crate::servicos::pedido_servico;
use rocket::response::Redirect;

#[get("/comprar/<cliente_id>")]
pub fn index(cliente_id: u32) -> Template {
    let produtos = produto_servico::listar();
    Template::render("compras/index", context!{ produtos: &produtos, cliente_id: cliente_id })
}

#[get("/adicionar_ao_pedido/<cliente_id>/<produto_id>")]
pub fn adicionar(cliente_id: u32, produto_id: u32) -> Redirect {
    // Adicionar produto ao pedido
    if pedido_servico::adicionar(cliente_id, produto_id) {
        // Redirecionar para a página do carrinho
        return Redirect::to(format!("/carrinho/{}", cliente_id));
    } else {
        // Se houver erro ao adicionar o produto ao pedido, redirecionar para a página inicial
        return Redirect::to("/");
    }
}

#[get("/carrinho/<cliente_id>")]
pub fn carrinho(cliente_id: u32) -> Template {
    let pedido = pedido_servico::ativo(cliente_id);
    Template::render("compras/carrinho", context!{ pedido: &pedido, cliente_id: cliente_id })
}


#[post("/pedidos/excluir-item/<cliente_id>/<pedido_id>/<produto_id>")]
pub fn excluir_item(
    cliente_id: u32,
    pedido_id: u32,
    produto_id: u32
) -> Redirect {
    pedido_servico::remover_produto(pedido_id, produto_id);
    Redirect::to(format!("/carrinho/{}", cliente_id))
}