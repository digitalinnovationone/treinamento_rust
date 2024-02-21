use rocket_dyn_templates::{Template, context};

use crate::models::cliente::Cliente;
use crate::servicos::cliente_servico;

use rocket::form::Form;
use rocket::response::{Redirect, Flash};
use rocket::request::FlashMessage;


fn processar_novo_cliente(cliente: NovoCliente) -> Result<(), ()> {
    if cliente.nome == "Cliente Teste" {
        Ok(())
    } else {
        Err(())
    }
}

fn atualizar_cliente(id: u32, cliente: NovoCliente) -> Result<(), ()> {
    println!("{}", id);
    
    if cliente.nome == "Cliente Teste" {
        Ok(())
    } else {
        Err(())
    }
}

fn excluir_cliente(id: u32) -> Result<(), ()> {
    if id == 1 {
        Ok(())
    } else {
        Err(())
    }
}


#[derive(FromForm)]
pub struct NovoCliente {
    nome: String,
    cpf: String,
}

#[get("/clientes")]
pub fn index() -> Template {
    let clientes = cliente_servico::get_clientes();
    Template::render("clientes/index", context! { area: "clientes", clientes: &clientes })
}

#[post("/clientes/novo", data = "<cliente_form>")]
pub fn cadastrar(cliente_form: Form<NovoCliente>) -> Result<Redirect, Flash<Redirect>> {
    let cliente = cliente_form.into_inner();
    
    println!("Nome do cliente: {}", cliente.nome);
    println!("Telefone do cliente: {}", cliente.cpf);

    if processar_novo_cliente(cliente).is_ok() {
        Ok(Redirect::to("/clientes"))
    } else {
        Err(Flash::error(
            Redirect::to("/clientes/novo"),
            "Erro ao cadastrar o cliente",
        ))
    }
}

#[get("/clientes/novo")]
pub fn novo(flash: Option<FlashMessage<'_>>) -> Template {
    let mut erro = "".to_string();
    if let Some(msg) = flash {
        if msg.kind() == "error" {
            erro = msg.message().to_string();
        }
    }

    
    let cliente = Cliente { id: 0, nome: "".to_string(), cpf: "".to_string() };
    Template::render("clientes/salvar", context! { 
        area: "clientes",
        action: "/clientes/novo",
        titulo: "Novo",
        cliente: &cliente,
        erro: erro
    })
}

#[get("/clientes/<id>/alterar")]
pub fn editar(id: u32, flash: Option<FlashMessage<'_>>) -> Template {
    let mut erro = "".to_string();
    if let Some(msg) = flash {
        if msg.kind() == "error" {
            erro = msg.message().to_string();
        }
    }


    let cliente = Cliente { id: id, nome: "Cliente 1".to_string(), cpf: "000.000.000-01".to_string() };
    Template::render("clientes/salvar", context! { 
        area: "clientes",
        action: format!("/clientes/{}/alterar", id),
        titulo: "Alterar",
        cliente: &cliente,
        erro: erro
    })
}


#[post("/clientes/<id>/alterar", data = "<cliente_form>")]
pub fn alterar(id: u32, cliente_form: Form<NovoCliente>) -> Result<Redirect, Flash<Redirect>> {
    let cliente = cliente_form.into_inner();

    if atualizar_cliente(id, cliente).is_ok() {
        Ok(Redirect::to("/clientes"))
    } else {
        Err(Flash::error(
            Redirect::to(format!("/clientes/{}/alterar", id)),
            "Erro ao atualizar o cliente",
        ))
    }
}

#[post("/clientes/<id>/excluir")]
pub fn excluir(id: u32) -> Result<Redirect, Flash<Redirect>> {

    if excluir_cliente(id).is_ok() {
        Ok(Redirect::to("/clientes"))
    } else {
        Err(Flash::error(
            Redirect::to(format!("/clientes/{}/alterar", id)),
            "Erro ao excluir o cliente",
        ))
    }
}

