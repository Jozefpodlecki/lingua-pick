use jsonwebtoken::{EncodingKey, Header, decode, Validation, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::env;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub user_name: Box<str>,
    pub iss: Box<str>,
    pub company: Box<str>,
    pub iat: usize,   
    pub exp: usize,
    pub role: Option<Box<str>>,
    pub session_id: Option<Box<str>>,
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new() -> Result<Self> {

        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            "nxN6LIHkXbhE+pFjzipwu14cqfA9Z4IU2axPvnIJ4VXDiCFvmtQwe+8CbLFYlNo+c fUKyC2ZYLeSutEThlwi+QE=".to_string()
        });

        let encoding_key = EncodingKey::from_base64_secret(&secret)?;
        let decoding_key = DecodingKey::from_base64_secret(&secret)?;

        Ok(Self {
            encoding_key,
            decoding_key,
        })
    }

    pub fn encode(&self, claims: JwtClaims) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(&Header::default(), &claims, &self.encoding_key)
    }

    pub fn decode(&self, token: &str) -> Result<TokenData<JwtClaims>, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode::<JwtClaims>(token, &self.decoding_key, &Validation::default())
    }
}