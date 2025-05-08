use super::error::ToApiError;
use super::json_stream::JsonStream;
use crate::{
    response::{BalanceResp, ChatResponse, ModelResp},
    RequestBuilder,
};
use anyhow::Result;
use reqwest::Client as ReqwestClient;

#[derive(Clone)]
/// A client for interacting with the DeepSeek API.
///
/// # Example
///
/// ```no_run
/// #[tokio::main]
/// async fn main() {
///     use deepseek_api::DeepSeekClientBuilder;
///
///     let api_key = "your_api_key".to_string();
///     let client = DeepSeekClientBuilder::new(api_key).build().unwrap();
///
///     // Get available models
///     let models = client.models().await.unwrap();
///
///     // Get user balance
///     let balance = client.balance().await.unwrap();
/// }
/// ```
///
/// # Fields
///
/// * `client` - The underlying HTTP client.
/// * `host` - The base URL for the DeepSeek API.
pub struct DeepSeekClient {
    pub(crate) client: ReqwestClient,
    pub(crate) host: String,
}

impl DeepSeekClient {
    /// Retrieves the list of available models from the DeepSeek API.
    ///
    /// This method sends a GET request to the `/models` endpoint of the DeepSeek API
    /// and returns a `Result` containing a `ModelResp` on success.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or if the response
    /// cannot be deserialized into a `ModelResp`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     use deepseek_api::DeepSeekClientBuilder;
    ///
    ///     let api_key = "your_api_key".to_string();
    ///     let client = DeepSeekClientBuilder::new(api_key).build().unwrap();
    ///     let models = client.models().await.unwrap();
    ///     println!("{:?}", models);
    /// }
    /// ```
    ///
    /// For more information, see the [DeepSeek API documentation](https://api-docs.deepseek.com/zh-cn/api/list-models).
    pub async fn models(&self) -> Result<ModelResp> {
        Ok(self
            .client
            .get(self.host.to_owned() + "/models")
            .send()
            .await?
            .json()
            .await?)
    }

    /// Retrieves the balance information of the user from the DeepSeek API.
    ///
    /// This method sends a GET request to the `/user/balance` endpoint of the DeepSeek API
    /// and returns a `Result` containing a `BalanceResp` on success.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or if the response
    /// cannot be deserialized into a `BalanceResp`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     use deepseek_api::DeepSeekClientBuilder;
    ///
    ///     let api_key = "your_api_key".to_string();
    ///     let client = DeepSeekClientBuilder::new(api_key).build().unwrap();
    ///     let balance = client.balance().await.unwrap();
    ///     println!("{:?}", balance);
    /// }
    /// ```
    ///
    /// For more information, see the [DeepSeek API documentation](https://api-docs.deepseek.com/zh-cn/api/get-user-balance).
    pub async fn balance(&self) -> Result<BalanceResp> {
        Ok(self
            .client
            .get(self.host.to_owned() + "/user/balance")
            .send()
            .await?
            .json()
            .await?)
    }

    /// Sends a completion request to the DeepSeek API.
    ///
    /// This method constructs a request using the provided `RequestBuilder` and sends it
    /// to the appropriate endpoint of the DeepSeek API. Depending on whether the request
    /// is for a beta feature or not, it will target either the `/beta/completions` or
    /// `/chat/completions` endpoint. The response can be either a full response or a
    /// streaming response, based on the `stream` optional of the `RequestBuilder`.
    ///
    /// # Type Parameters
    ///
    /// * `Builder` - A type that implements the `RequestBuilder` trait, used to construct
    ///   the request payload.
    ///
    /// # Arguments
    ///
    /// * `request_builder` - An instance of a type implementing the `RequestBuilder` trait,
    ///   which is used to build the request payload.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `ChatResponse` on success. The `ChatResponse` can either be
    /// a full response or a streaming response, depending on the request.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The request fails to send.
    /// - The response contains an API error.
    /// - The response cannot be deserialized into the expected type.
    ///
    /// # Example
    ///
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     use deepseek_api::{request::{MessageRequest, UserMessageRequest}, DeepSeekClientBuilder, CompletionsRequestBuilder};
    ///     use deepseek_api::response::ChatResponse;
    ///     use futures_util::StreamExt;
    ///
    ///     let api_key = "your_api_key".to_string();
    ///     let client = DeepSeekClientBuilder::new(api_key).build().unwrap();
    ///     let request_builder = CompletionsRequestBuilder::new(vec![MessageRequest::User(
    ///                 UserMessageRequest::new("Hello, DeepSeek!")
    ///     )]);
    ///
    ///     let response = client.send_completion_request(request_builder).await.unwrap();
    ///     match response {
    ///         ChatResponse::Full(full_response) => println!("{:?}", full_response),
    ///         ChatResponse::Stream(mut stream) => {
    ///             while let Some(item) = stream.next().await {
    ///                 println!("{:?}", item);
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// For more information, see the [DeepSeek API documentation](https://api-docs.deepseek.com/zh-cn/api).
    pub async fn send_completion_request<Builder>(
        &self,
        request_builder: Builder,
    ) -> Result<ChatResponse<Builder::Response, Builder::Item>>
    where
        Builder: RequestBuilder + Send + Sized,
    {
        let host = if request_builder.is_beta() {
            self.host.to_owned() + "/beta/completions"
        } else {
            self.host.to_owned() + "/chat/completions"
        };
        let is_stream = request_builder.is_stream();

        let request = request_builder.build();
        let resp = self
            .client
            .post(&host)
            .json(&request)
            .send()
            .await?
            .to_api_err()
            .await?;
        if is_stream {
            Ok(ChatResponse::Stream(JsonStream::new(resp)))
        } else {
            Ok(ChatResponse::Full(resp.json().await?))
        }
    }
}
