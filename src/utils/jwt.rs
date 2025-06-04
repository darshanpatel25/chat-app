// utils/jwt.rs
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,           // user_id
    pub role: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: i64, role: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        role: role.to_string(),
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).unwrap()
}

pub fn verify_jwt(token: &str) -> Option<Claims> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .ok()
}
