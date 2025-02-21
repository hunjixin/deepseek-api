use std::fmt;

pub enum ApiError {
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

impl fmt::Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (Code: {:?})", self, self)
    }
}

impl std::error::Error for ApiError {}

pub trait ToApiError {
    async fn to_api_err(self) -> Result<reqwest::Response, ApiError>;
}

impl ToApiError for reqwest::Response {
     async fn to_api_err(self) -> Result<reqwest::Response, ApiError> {
        let status = self.status().as_u16();
        match status {
            400 | 401 | 402 | 403 | 429 | 500 | 503 => {
                let message = self.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(match status {
                    400 => ApiError::BadRequest(message),
                    401 => ApiError::Unauthorized(message),
                    402 => ApiError::InsufficientFunds(message),
                    403 => ApiError::InvalidParameters(message),
                    429 => ApiError::RateLimitExceeded(message),
                    500 => ApiError::ServerError(message),
                    503 => ApiError::ServiceUnavailable(message),
                    _ => unreachable!(),
                })
            }
            _ => Ok(self),
        }    
    }
}