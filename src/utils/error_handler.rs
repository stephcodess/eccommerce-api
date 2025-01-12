use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

/// Enum representing various application-level errors.
#[derive(Debug, Display)]
pub enum ErrorHandler {
    /// Internal server error with a generic message.
    #[display("Internal Server Error")]
    InternalServerError,

    /// Bad request error with a custom message.
    #[display("Bad Request: {}", _0)]
    BadRequest(String),

    /// Error fetching JSON Web Key Set (JWKS).
    #[display("JWKS Fetch Error")]
    JWKSFetchError,
}

/// Implementation of the `ResponseError` trait to convert custom errors into HTTP responses.
impl ResponseError for ErrorHandler {
    fn error_response(&self) -> HttpResponse {
        match self {
            ErrorHandler::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error. Please try again later.")
            }
            ErrorHandler::BadRequest(message) => {
                HttpResponse::BadRequest().json(message)
            }
            ErrorHandler::JWKSFetchError => {
                HttpResponse::InternalServerError().json("Failed to fetch JWKS.")
            }
        }
    }
}
