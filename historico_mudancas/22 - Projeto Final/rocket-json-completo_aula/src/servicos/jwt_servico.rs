// src/servicos/auth_servico.rs

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (usuário identificado pelo token)
    exp: usize, // Expiry (tempo de expiração do token)
}

pub fn gerar_token_jwt(adm_id: u32) -> String {
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::try_hours(24).expect("valid duration"))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: adm_id.to_string(),
        exp: expiration_time as usize,
    };

    let secret = env::var("SECRET_JWT").unwrap_or_else(|_| "your_secret_key".to_string());
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn verifica_token(token: &str) -> bool {
    let secret = env::var("SECRET_JWT").unwrap_or_else(|_| "your_secret_key".to_string());
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).is_ok()
}
