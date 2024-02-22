use rocket::{http::Method, Request, Data, fairing::{Fairing, Info, Kind}, http::uri::Origin};
use crate::servicos::auth_servico::verify_token;

pub struct JwtFairing;

#[rocket::async_trait]
impl Fairing for JwtFairing {
    fn info(&self) -> Info {
        Info {
            name: "JWT Authentication Fairing",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        // Lista de rotas que não requerem autenticação
        let open_routes = ["/logar", "/"];

        let request_path = request.uri().path();
        if open_routes.contains(&request_path.as_str()) {
            return; 
        }

        let token_valid = request.headers().get_one("Authorization")
            .and_then(|header| header.strip_prefix("Bearer "))
            .map(|token| verify_token(token))
            .unwrap_or(false);

        if !token_valid {
            if let Ok(uri) = Origin::parse("/unauthorized") {
                request.set_uri(uri);
                request.set_method(Method::Get);
            }
        }
    }
}
