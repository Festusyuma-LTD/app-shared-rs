use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use serde_json::json;

const SERVER_ERROR_MSG: &str = "Internal server error";

#[derive(Deserialize, Debug)]
pub enum ServiceError {
    NotFound,
    ServerError,
    Http(u16),
    HttpMessage(u16, String),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ServiceError::NotFound => (StatusCode::NOT_FOUND, String::from("Not found")),
            ServiceError::ServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, SERVER_ERROR_MSG.into())
            }
            ServiceError::Http(code) => (
                StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                SERVER_ERROR_MSG.into(),
            ),
            ServiceError::HttpMessage(code, message) => (
                StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                message,
            ),
        };

        let body = Json(json!({ "error": message }));

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for ServiceError {
    fn from(error: anyhow::Error) -> Self {
        println!("unknown error occurred: {:?}", error);
        ServiceError::ServerError
    }
}
