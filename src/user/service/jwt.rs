use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

const SECRET: &str = "ling0017";

impl super::Service {
    pub fn create_token(&self, user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
        let my_claims = Claims {
            sub: user_id.to_string(),
            exp: 10000000000,
        };
        let token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(SECRET.as_ref()),
        )?;
        Ok(token)
    }

    pub fn verify_token(token: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(token_data.claims.sub)
    }
}
