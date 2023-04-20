use axum::{
    response::{IntoResponse, Response},
    Json,
};
use error_stack::Report;
use http::StatusCode;
use serde::Serialize;

#[derive(Debug)]
pub struct OurReport(error_stack::Report<ConduitError>);

pub type ConduitResult<T> = std::result::Result<T, OurReport>;

#[derive(thiserror::Error, Debug)]
pub enum ConduitError {
    #[error("authentication is required to access this resource")]
    Unauthorized,
    #[error("unexpected error has occurred")]
    InternalServerError,
    #[error("unexpected error: {0}")]
    InternalServerErrorWithCtx(String),
    #[error("wrong email or password")]
    InvalidCredential,
    #[error("user already existed")]
    ExistedUser,

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

impl IntoResponse for OurReport {
    fn into_response(self) -> Response {
        let err: &ConduitError = self.0.downcast_ref().unwrap();

        let status = match err {
            ConduitError::Unauthorized => StatusCode::UNAUTHORIZED,
            ConduitError::InvalidCredential | ConduitError::ExistedUser => StatusCode::BAD_REQUEST,

            ConduitError::InternalServerError
            | ConduitError::InternalServerErrorWithCtx(_)
            | ConduitError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (
            status,
            Json(ResponseMsg {
                message: err.to_string(),
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
struct ResponseMsg {
    message: String,
}

impl<E> From<E> for OurReport
where
    E: Into<ConduitError>,
{
    fn from(value: E) -> Self {
        let err: ConduitError = value.into();
        let tp: Report<ConduitError> = error_stack::report!(err);
        OurReport(tp)
    }
}
