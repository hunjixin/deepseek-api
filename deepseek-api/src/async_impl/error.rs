use crate::error::ApiError;
use reqwest::Response;

pub trait ToApiError {
    fn to_api_err(self) -> impl std::future::Future<Output = Result<Response, ApiError>> + Send;
}

impl ToApiError for Response {
    async fn to_api_err(self) -> Result<Response, ApiError> {
        let status = self.status().as_u16();
        match status {
            400 | 401 | 402 | 422 | 429 | 500 | 503 => {
                let message = self
                    .text()
                    .await
                    .map_err(|err| ApiError::Unknown(err.to_string()))?;
                Err(match status {
                    400 => ApiError::BadRequest(message),
                    401 => ApiError::Unauthorized(message),
                    402 => ApiError::InsufficientFunds(message),
                    422 => ApiError::InvalidParameters(message),
                    429 => ApiError::RateLimitExceeded(message),
                    500 => ApiError::ServerError(message),
                    503 => ApiError::ServiceUnavailable(message),
                    code => ApiError::Unknown(format!(
                        "Response error not in document{}: {}",
                        code, message
                    )),
                })
            }
            _ => Ok(self),
        }
    }
}
