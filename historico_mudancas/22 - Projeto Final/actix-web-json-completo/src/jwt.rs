use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm, errors::Error as JwtError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_token(claims: Claims, secret: &str) -> Result<String, JwtError> {
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    encode(&Header::new(Algorithm::HS256), &claims, &encoding_key)
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, JwtError> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256)).map(|data| data.claims)
}
