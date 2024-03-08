use rocket_dyn_templates::{Template, context};
use crate::servicos::cliente_servico;

use rocket::form::Form;
use rocket::response::{Redirect, Flash};
use crate::dtos::cliente_dto::ClienteDto;
use rocket::request::FlashMessage;

#[get("/clientes")]
pub fn index() -> Template {
    let clientes = cliente_servico::listar();
    Template::render("clientes/index", context!{ clientes: &clientes })
}

#[get("/clientes/novo")]
pub fn novo(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("clientes/novo", context!{ erro: erro_flash(flash) })
}

#[post("/clientes/criar", data = "<cliente_dto_form>")]
pub fn criar(cliente_dto_form: Form<ClienteDto>) -> Result<Redirect, Flash<Redirect>>  {
    let cliente_dto = cliente_dto_form.into_inner();

    if cliente_servico::criar(cliente_dto.nome, cliente_dto.telefone) {
        Ok(Redirect::to("/clientes"))
    } else {
        Err(Flash::error(
            Redirect::to("/clientes/novo"),
            "Erro ao cadastrar cliente",
        ))
    }
}

#[get("/clientes/<id>/editar")]
pub fn editar(id: u32, flash: Option<FlashMessage<'_>>) -> Template  {
    let cliente = cliente_servico::buscar_por_id(id);
    Template::render("clientes/editar", context! { 
        cliente: &cliente,
        erro: erro_flash(flash)
    })
}

#[post("/clientes/<id>/alterar", data = "<cliente_dto_form>")]
pub fn alterar(id: u32, cliente_dto_form: Form<ClienteDto>) -> Result<Redirect, Flash<Redirect>> {
    let cliente_dto = cliente_dto_form.into_inner();
    
    if cliente_servico::alterar(id, cliente_dto.nome, cliente_dto.telefone) {
        Ok(Redirect::to("/clientes"))
    } else {
        Err(Flash::error(
            Redirect::to(format!("/clientes/{}/editar", id)),
            "Erro ao alterar cliente",
        ))
    }
}


#[get("/clientes/<id>/excluir")]
pub fn excluir(id: u32) -> Result<Redirect, Flash<Redirect>> {
    if cliente_servico::excluir_por_id(id) {
        Ok(Redirect::to("/clientes"))
    } else {
        Err(Flash::error(
            Redirect::to(format!("/clientes/{}/editar", id)),
            "Erro ao excluir cliente",
        ))
    }
}


fn erro_flash(flash: Option<FlashMessage<'_>>) -> String {
    let mut erro = "".to_string();
    if let Some(msg) = flash {
        if msg.kind() == "error" {
            erro = msg.message().to_string();
        }
    }

    erro
}
