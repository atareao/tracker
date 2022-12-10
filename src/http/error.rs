use serde::Serialize;
use axum::{
    response::{
        Response,
        IntoResponse
    },
    http::StatusCode,
    Json
};


#[derive(Debug)]
pub enum LocationError {
    ReadError,
    NotFound,
    Sqlx(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse{
    code: u16,
    message: String,
}

impl LocationError{
    fn status_code(&self) -> StatusCode{
        match self{
            Self::ReadError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Sqlx(value) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn get(&self) -> ErrorResponse{
        match self{
            LocationError::ReadError => ErrorResponse{
                code: self.status_code().as_u16(),
                message: "Internal error".to_string(),
            },
            LocationError::NotFound => ErrorResponse{
                code: self.status_code().as_u16(),
                message: "NotFound".to_string(),
            },
            LocationError::Sqlx(message) => ErrorResponse{
                code: self.status_code().as_u16(),
                message: message.to_string(),
            }
        }
    }
}
impl IntoResponse for LocationError{
    fn into_response(self) -> Response {
        (self.status_code(), Json(self.get())).into_response()
    }
}
