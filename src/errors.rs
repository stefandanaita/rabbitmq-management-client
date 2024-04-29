use http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum RabbitMqClientError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Invalid RabbitMq API url: {0}")]
    InvalidApiUrl(String),
    #[error("Failed to parse the API response: {0}")]
    ParsingError(#[source] reqwest::Error),
    #[error("Failed to execute the middleware: {0}")]
    Middleware(#[source] anyhow::Error),
    #[error("Failed to send the API request: {0}")]
    Request(#[source] reqwest::Error),
    #[error("RabbitMq API error: {0:?}")]
    ApiError(RabbitMqApiError),
    #[error("Unexpected API response: {0}")]
    UnexpectedResponse(String),
}

impl From<reqwest_middleware::Error> for RabbitMqClientError {
    fn from(value: reqwest_middleware::Error) -> Self {
        match value {
            reqwest_middleware::Error::Middleware(e) => RabbitMqClientError::Middleware(e),
            reqwest_middleware::Error::Reqwest(e) => RabbitMqClientError::Request(e),
        }
    }
}

#[derive(Debug)]
pub struct RabbitMqApiError {
    pub code: StatusCode,
    pub text: String,
}
