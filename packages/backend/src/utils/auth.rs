// src/utils/auth.rs
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

lazy_static! {
    static ref SECRET_KEY: String =
        env::var("SECRET_KEY").unwrap_or_else(|_| "some_secret".repeat(8));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn create_token(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;
    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    )
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
