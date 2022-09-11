use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct MessageResponse {
    pub messages: Vec<String>,
}

impl From<Vec<String>> for MessageResponse {
    fn from(messages: Vec<String>) -> Self {
        Self { messages }
    }
}

impl From<String> for MessageResponse {
    fn from(message: String) -> Self {
        Self::from(vec![message])
    }
}

impl From<&str> for MessageResponse {
    fn from(message: &str) -> Self {
        Self::from(String::from(message))
    }
}

impl IntoResponse for MessageResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize)]
pub struct DataResponse<T: Serialize> {
    pub data: T,
    pub messages: Vec<String>,
}

impl<T: Serialize> From<T> for DataResponse<T> {
    fn from(data: T) -> Self {
        Self {
            data,
            messages: Vec::new(),
        }
    }
}

impl<T: Serialize> IntoResponse for DataResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl<T: Serialize> DataResponse<T> {
    pub fn with_message(data: T, message: String) -> Self {
        Self {
            data: data,
            messages: vec![message],
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse<T: serde::Serialize> {
    pub messages: Vec<String>,
    pub data: Option<T>,
}

impl<T: Serialize> From<Vec<String>> for ErrorResponse<T> {
    fn from(errors: Vec<String>) -> Self {
        Self {
            messages: errors,
            data: None,
        }
    }
}

impl<T: Serialize> From<String> for ErrorResponse<T> {
    fn from(error: String) -> Self {
        Self::from(vec![error])
    }
}

impl<T: Serialize> From<&str> for ErrorResponse<T> {
    fn from(error: &str) -> Self {
        Self::from(String::from(error))
    }
}

impl<T: Serialize> IntoResponse for ErrorResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl<T: Serialize> ErrorResponse<T> {
    pub fn with_data(errors: Vec<String>, data: T) -> Self {
        Self {
            messages: errors,
            data: Some(data),
        }
    }
}
