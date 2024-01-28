use super::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HTTPError {
    pub code: u16,
    pub message: String,
}

impl HTTPError {
    pub fn new(error: Error) -> Self {
        match error {
            Error::BadRequest(detail) => HTTPError {
                code: 400,
                message: format!("Bad request: {}", detail),
            },
            Error::InternalServer(msg) => HTTPError {
                code: 500,
                message: format!("Internal server error: {}", msg),
            },
            Error::NotFound(resource) => HTTPError {
                code: 404,
                message: format!("{} not found", resource),
            },
            Error::Unauthorized(operation) => HTTPError {
                code: 401,
                message: format!("Unauthorized: {}", operation),
            },
            Error::Validation { field, message } => HTTPError {
                code: 400,
                message: format!("Validation error for field {}: {}", field, message),
            },
            Error::Conflict(resource) => HTTPError {
                code: 409,
                message: format!("{} already exists", resource),
            },
            Error::MaliciousPrompt(resource) => HTTPError {
                code: 422,
                message: format!("Malicious prompt: {}", resource),
            },
            // Handle the additional errors here:
            Error::Render(detail) => HTTPError {
                code: 500,
                message: format!("Render error: {}", detail),
            },
            Error::Template(detail) => HTTPError {
                code: 500,
                message: format!("Template error: {}", detail),
            },
            Error::Parse(detail) => HTTPError {
                code: 400,
                message: format!("Parse error: {}", detail),
            },
        }
    }
}
