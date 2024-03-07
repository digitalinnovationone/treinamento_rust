use crate::model_views::administrador_token::AdministradorToken;
use crate::servicos::jwt_servico::gerar_token_jwt;
use crate::repositorios::administrador_repositorio;
use crate::config::cnn;

pub fn login(email: String, senha: String) -> Result<AdministradorToken, String> {
    let conn = cnn::establish_connection();
    let adm = administrador_repositorio::buscar_por_email_senha(&conn, &email, &senha);
    match adm {
        Ok(adm) => Ok(
            AdministradorToken {
                id: adm.id,
                nome: adm.nome,
                email: adm.email,
                token: gerar_token_jwt(adm.id),
            }
        ),
        Err(_) => Err("Usuário ou senha não encontrado".to_string()),
    }
}
