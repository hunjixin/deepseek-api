use std::fmt;
#[derive(Debug)]
pub enum ApiError {
    Unknown(String),
    BadRequest(String),
    Unauthorized(String),
    InsufficientFunds(String),
    InvalidParameters(String),
    RateLimitExceeded(String),
    ServerError(String),
    ServiceUnavailable(String),
}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            ApiError::Unknown(msg) => format!("Unknown Error: {}", msg),
            ApiError::BadRequest(msg) => format!("Bad Request: {}", msg),
            ApiError::Unauthorized(msg) => format!("Unauthorized: {}", msg),
            ApiError::InsufficientFunds(msg) => format!("Insufficient Funds: {}", msg),
            ApiError::InvalidParameters(msg) => format!("Invalid Parameters: {}", msg),
            ApiError::RateLimitExceeded(msg) => format!("Rate Limit Exceeded: {}", msg),
            ApiError::ServerError(msg) => format!("Server Error: {}", msg),
            ApiError::ServiceUnavailable(msg) => format!("Service Unavailable: {}", msg),
        };
        write!(f, "{}", description)
    }
}

impl std::error::Error for ApiError {}
