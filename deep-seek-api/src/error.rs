use std::fmt;

pub enum ApiError {
    BadRequest(u16),
    Unauthorized(u16),
    InsufficientFunds(u16),
    InvalidParameters(u16),
    RateLimitExceeded(u16),
    ServerError(u16),
    ServiceUnavailable(u16),
}

impl fmt::Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match *self {
            ApiError::BadRequest(_) => "Bad Request: Request body format error.",
            ApiError::Unauthorized(_) => "Unauthorized: API key error, authentication failed.",
            ApiError::InsufficientFunds(_) => {
                "Insufficient Funds: Account balance is insufficient."
            }
            ApiError::InvalidParameters(_) => "Invalid Parameters: Request body parameter error.",
            ApiError::RateLimitExceeded(_) => {
                "Rate Limit Exceeded: Request rate (TPM or RPM) has reached the limit."
            }
            ApiError::ServerError(_) => "Server Error: Server internal failure.",
            ApiError::ServiceUnavailable(_) => "Service Unavailable: Server overload.",
        };

        write!(f, "{} (Code: {:?})", description, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_debug() {
        let test_cases = vec![
            (
                ApiError::BadRequest(400),
                "Bad Request: Request body format error.",
                "BadRequest(400)",
            ),
            (
                ApiError::Unauthorized(401),
                "Unauthorized: API key error, authentication failed.",
                "Unauthorized(401)",
            ),
            (
                ApiError::InsufficientFunds(402),
                "Insufficient Funds: Account balance is insufficient.",
                "InsufficientFunds(402)",
            ),
            (
                ApiError::InvalidParameters(422),
                "Invalid Parameters: Request body parameter error.",
                "InvalidParameters(422)",
            ),
            (
                ApiError::RateLimitExceeded(429),
                "Rate Limit Exceeded: Request rate (TPM or RPM) has reached the limit.",
                "RateLimitExceeded(429)",
            ),
            (
                ApiError::ServerError(500),
                "Server Error: Server internal failure.",
                "ServerError(500)",
            ),
            (
                ApiError::ServiceUnavailable(503),
                "Service Unavailable: Server overload.",
                "ServiceUnavailable(503)",
            ),
        ];

        for (error, description, code) in test_cases {
            let debug_output = format!("{:?}", error);
            assert!(debug_output.contains(description));
            assert!(debug_output.contains(code));
        }
    }
}
