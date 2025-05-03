use crate::completions::ChatCompletions;
use crate::response::{BalanceResp, ModelResp};
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
///     use deepseek_api::ClientBuilder;
///
///     let api_key = "your_api_key".to_string();
///     let client = ClientBuilder::new(api_key).build().unwrap();
///
///     // Get available models
///     let models = client.models().await.unwrap();
///
///     // Get user balance
///     let balance = client.balance().await.unwrap();
///
///     // Create a chat completion
///     let chat = client.chat();
/// }
/// ```
///
/// # Fields
///
/// * `client` - The underlying HTTP client.
/// * `host` - The base URL for the DeepSeek API.
pub struct Client {
    pub(crate) client: ReqwestClient,
    pub(crate) host: String,
}

impl Client {
    pub fn chat(&self) -> ChatCompletions {
        ChatCompletions {
            client: self.client.clone(),
            host: self.host.clone(),
        }
    }

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
    ///     use deepseek_api::ClientBuilder;
    ///
    ///     let api_key = "your_api_key".to_string();
    ///     let client = ClientBuilder::new(api_key).build().unwrap();
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
    ///     use deepseek_api::ClientBuilder;
    ///
    ///     let api_key = "your_api_key".to_string();
    ///     let client = ClientBuilder::new(api_key).build().unwrap();
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
}
