use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{self, Ready};
use std::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;
use crate::jwt::decode_token;
use crate::config;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization").map(|hv| hv.to_str().unwrap_or("").to_string());
        let fut = self.service.call(req);

        Box::pin(async move {
            // Usa o valor clonado de "Authorization"
            if let Some(auth_str) = auth_header {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str["Bearer ".len()..];
                    
                    let config = config::load_config().expect("Failed to load configuration.");
                    let jwt_secret = config.jwt.secret.clone();
    
                    // Tentativa de decodificação do token
                    match decode_token(token, &jwt_secret) {
                        Ok(_claims) => {
                            // Se o token for válido, prossegue com a requisição
                            return fut.await;
                        },
                        Err(_) => {
                            // Se o token for inválido, retorna uma resposta 401 Unauthorized
                            return Err(actix_web::error::ErrorUnauthorized("Token inválido"));
                        }
                    }
                }
            }
    
            // Se o cabeçalho "Authorization" não estiver presente ou o token não começar com "Bearer ",
            // ou qualquer erro ao extrair o valor do cabeçalho, retorna uma resposta 401 Unauthorized
            Err(actix_web::error::ErrorUnauthorized("Token obrigatório"))
        })
    }
}
