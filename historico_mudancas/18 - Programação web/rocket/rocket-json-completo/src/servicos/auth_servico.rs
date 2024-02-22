// src/servicos/auth_servico.rs

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (usuário identificado pelo token)
    exp: usize, // Expiry (tempo de expiração do token)
}

pub fn generate_token(user_id: &str) -> String {
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration_time as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret("your_secret_key".as_ref())).unwrap()
}

pub fn verify_token(token: &str) -> bool {
    decode::<Claims>(token, &DecodingKey::from_secret("your_secret_key".as_ref()), &Validation::default()).is_ok()
}
