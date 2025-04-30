use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde_json::json;

#[derive(Debug, Display)]
pub enum ApiError {
    #[display(fmt = "Player not found.")]
    NotFound,
    #[display(fmt = "Invalid input.")]
    BadRequest,
    #[display(fmt = "Internal server error.")]
    InternalError,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound => HttpResponse::NotFound().json(json!({ "error": "Player not found." })),
            ApiError::BadRequest => HttpResponse::BadRequest().json(json!({ "error": "Invalid input." })),
            ApiError::InternalError => HttpResponse::InternalServerError().json(json!({ "error": "Internal server error." })),
        }
    }
}
