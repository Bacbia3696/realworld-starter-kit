use axum::{
    extract::FromRequestParts,
    headers::{authorization::Credentials, Authorization},
    Extension, RequestPartsExt, TypedHeader,
};
use http::{request::Parts, HeaderValue};

use crate::{
    errors::{ConduitError, OurReport},
    jwt,
};

#[derive(Debug)]
pub struct UserID(pub i64);

#[derive(Clone, PartialEq, Debug)]
/// Token holder for Bearer Authentication, most often seen with oauth
pub struct Token(HeaderValue);

impl Token {
    /// View the token part as a `&str`.
    pub fn token(&self) -> &str {
        &self.0.to_str().unwrap()["Token ".len()..]
    }
}

impl Credentials for Token {
    const SCHEME: &'static str = "Token";

    fn decode(value: &HeaderValue) -> Option<Self> {
        debug_assert!(
            value.as_bytes().starts_with(b"Token "),
            "HeaderValue to decode should start with \"Token ..\", received = {:?}",
            value,
        );

        Some(Token(value.clone()))
    }

    fn encode(&self) -> HeaderValue {
        (&self.0).into()
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for UserID
where
    S: Send + Sync,
{
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection = OurReport;

    /// Perform the extraction.
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(token_service): Extension<jwt::JwtService> =
            Extension::from_request_parts(parts, state)
                .await
                .map_err(|err| ConduitError::InternalServerErrorWithCtx(err.to_string()))?;
        // extract token from authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Token>>>()
            .await
            .map_err(|_| ConduitError::Unauthorized)?;
        let token = bearer.token();
        let user_id = token_service.get_user_id(token)?;
        Ok(UserID(user_id))
    }
}
