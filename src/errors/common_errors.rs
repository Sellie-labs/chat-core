use actix_web::HttpResponse;
use handlebars::{RenderError, TemplateError};
use serde_json::Error as SerdeJsonError;

use super::HTTPError;

#[derive(Debug, Clone)]
pub enum Error {
    InternalServer(String),
    NotFound(String),
    Unauthorized(String),
    Validation { field: String, message: String },
    Conflict(String),
    BadRequest(String),
    Render(String),
    Template(String),
    Parse(String),
    MaliciousPrompt(String),
}

impl Error {
    pub fn internal_server_error(message: String) -> Self {
        Error::InternalServer(message)
    }

    pub fn not_found_error(resource: String) -> Self {
        Error::NotFound(resource)
    }

    pub fn unauthorized_error(operation: String) -> Self {
        Error::Unauthorized(operation)
    }

    pub fn validation_error(field: String, message: String) -> Self {
        Error::Validation { field, message }
    }

    pub fn conflict_error(resource: String) -> Self {
        Error::Conflict(resource)
    }

    pub fn bad_request_error(detail: String) -> Self {
        Error::BadRequest(detail)
    }

    pub fn malicious_prompt_error(detail: String) -> Self {
        Error::MaliciousPrompt(detail)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadRequest(detail) => {
                write!(f, "Bad request: {}", detail)
            }

            Error::InternalServer(msg) => {
                write!(f, "Internal server error: {}", msg)
            }
            Error::NotFound(resource) => {
                write!(f, "{} not found", resource)
            }
            Error::Unauthorized(operation) => {
                write!(f, "Unauthorized: {}", operation)
            }
            Error::Validation { field, message } => {
                write!(f, "Validation error for field {}: {}", field, message)
            }
            Error::Conflict(resource) => {
                write!(f, "{} already exists", resource)
            }
            Error::Render(e) => write!(f, "Render error: {}", e),
            Error::Template(e) => write!(f, "Template error: {}", e),
            Error::Parse(msg) => write!(f, "Parsing error: {}", msg),
            Error::MaliciousPrompt(msg) => write!(f, "Malicious prompt error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let http_error = HTTPError::new(self.clone());
        let status_code = actix_web::http::StatusCode::from_u16(http_error.code)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        HttpResponse::build(status_code).json(http_error)
    }
}

impl From<RenderError> for Error {
    fn from(error: RenderError) -> Self {
        Error::Render(error.to_string())
    }
}

impl From<SerdeJsonError> for Error {
    fn from(error: SerdeJsonError) -> Self {
        Error::Parse(error.to_string())
    }
}

impl From<TemplateError> for Error {
    fn from(error: TemplateError) -> Self {
        Error::Template(error.to_string())
    }
}
