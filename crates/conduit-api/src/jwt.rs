use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::types::time::OffsetDateTime;

use crate::{
    config::AppConfig,
    errors::{ConduitError, ConduitResult},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: i64,
    pub exp: usize,
}

#[derive(Clone)]
pub struct JwtService {
    config: Arc<AppConfig>,
}

impl JwtService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }

    pub fn new_token(&self, user_id: i64, email: &str) -> ConduitResult<String> {
        let from_now = Duration::from_secs(3600);
        let expired_future_time = SystemTime::now() + from_now;
        let exp = OffsetDateTime::from(expired_future_time);

        let claims = Claims {
            sub: String::from(email),
            exp: exp.unix_timestamp() as usize,
            user_id,
        };

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.config.token_secret.as_bytes()),
        )
        .map_err(|err| ConduitError::InternalServerErrorWithCtx(err.to_string()))?;

        Ok(token)
    }

    pub fn get_user_id(&self, token: &str) -> ConduitResult<i64> {
        let decoded_token = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.token_secret.as_bytes()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .map_err(|err| ConduitError::InternalServerErrorWithCtx(err.to_string()))?;
        Ok(decoded_token.claims.user_id)
    }
}
