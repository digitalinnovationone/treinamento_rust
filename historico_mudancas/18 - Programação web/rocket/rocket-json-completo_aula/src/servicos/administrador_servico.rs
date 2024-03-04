use crate::model_views::administrador_token::AdministradorToken;
use crate::models::administrador::Administrador;
use crate::servicos::jwt_servico::gerar_token_jwt;


// -> https://chat.openai.com/share/cc0f1139-edc6-49a7-b58b-bb4cf17a001e

pub fn busca_administrador_por_email_senha(_email: String, _senha: String) -> Option<Administrador> {
    // Simulando buscar o administrador do banco de dados
    println!("{}", _email);
    println!("{}", _senha);
    
    let administrador_encontrado = _email == "danilo@teste.com" && _senha == "123456";

    if administrador_encontrado {
        Some(Administrador {
            id: 1,
            nome: "Danilo".to_string(),
            email: "danilo@teste.com".to_string(),
            senha: "123456".to_string(),
        })
    } else {
        None
    }
}

pub fn login(email: String, senha: String) -> Result<AdministradorToken, String> {
    let adm = busca_administrador_por_email_senha(email, senha);
    match adm {
        Some(adm) => Ok(
            AdministradorToken {
                id: adm.id,
                nome: adm.nome,
                email: adm.email,
                token: gerar_token_jwt(adm.id),
            }
        ),
        None => Err("Usuário ou senha não encontrado".to_string()),
    }
}
