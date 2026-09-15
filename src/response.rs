use crate::error::ServiceError;
use crate::response::ServiceResponse::Status;

use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use serde_json::json;

pub type ServiceResult<T> = Result<T, ServiceError>;

pub enum ServiceResponse<T: Serialize> {
    Ok(T),
    Status(T, u16),
    Err(ServiceError),
}

impl<T: Serialize> IntoResponse for ServiceResponse<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            ServiceResponse::Ok(body) => {
                let body = Json(json!({ "data": body }));
                (StatusCode::OK, body).into_response()
            }
            Status(body, status) => {
                let body = Json(json!({ "data": body }));
                let status = StatusCode::from_u16(status).unwrap_or(StatusCode::OK);

                (status, body).into_response()
            }
            ServiceResponse::Err(err) => err.into_response(),
        }
    }
}

impl<T: Serialize> ServiceResponse<T> {
    pub fn into_result(self) -> Result<T, ServiceError> {
        match self {
            ServiceResponse::Ok(data) => Ok(data),
            ServiceResponse::Status(data, _) => Ok(data),
            ServiceResponse::Err(err) => Err(err),
        }
    }
}

impl<T: Serialize> From<ServiceError> for ServiceResponse<T> {
    fn from(value: ServiceError) -> Self {
        ServiceResponse::Err(value)
    }
}

impl<T: Serialize> From<ServiceResult<T>> for ServiceResponse<T> {
    fn from(value: ServiceResult<T>) -> Self {
        match value {
            Ok(data) => ServiceResponse::Ok(data),
            Err(err) => ServiceResponse::Err(err),
        }
    }
}
