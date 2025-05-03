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

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::{Response, StatusCode};
    use http::response::Builder;
    use std::error::Error;

    fn mock_response(status: u16, body: &str) -> Response {
        let response = Builder::new()
            .status(status)
            .body(body.to_owned())
            .unwrap();
        Response::from(response)
    }

    #[tokio::test]
    async fn test_ok_responses() -> Result<(), Box<dyn Error>> {
        let ok_statuses = [200, 201, 302, 404];

        for status in ok_statuses {
            let resp = mock_response(status, "");
            let result = resp.to_api_err().await?;
            assert_eq!(result.status(), StatusCode::from_u16(status)?);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_error_mapping() {
        let test_cases = vec![
            (400, "bad request", ApiError::BadRequest("bad request".into())),
            (401, "unauthorized",ApiError::Unauthorized("unauthorized".into())),
            (402, "insufficient funds", ApiError::InsufficientFunds("insufficient funds".into())),
            (422, "invalid params",ApiError::InvalidParameters("invalid params".into())),
            (429, "rate limit", ApiError::RateLimitExceeded("rate limit".into())),
            (500, "server error", ApiError::ServerError("server error".into())),
            (503, "unavailable", ApiError::ServiceUnavailable("unavailable".into())),
        ];

        for (status, err_str, _expected_err) in test_cases {
            let resp = mock_response(status, err_str );
            let err = resp.to_api_err().await.unwrap_err();

            assert!(
                matches!(&err, _expected_err),
                "Status {} generated wrong error type: {:?}",
                status, err
            );
            assert_eq!(err.to_string(), _expected_err.to_string());
        }
    }
}
