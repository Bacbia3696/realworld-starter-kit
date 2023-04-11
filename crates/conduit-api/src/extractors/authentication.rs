use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    Extension, RequestPartsExt, TypedHeader,
};
use http::request::Parts;

use crate::{
    errors::{ConduitError, OurReport},
    jwt,
};

#[derive(Debug)]
pub struct UserID(i64);

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
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ConduitError::Unauthorized)?;
        let user_id = token_service.get_user_id(bearer.token())?;
        Ok(UserID(user_id))
    }
}
