#![feature(trait_alias)]

pub mod completions;
mod error;

pub mod json_stream;
pub mod request;
pub mod response;

use anyhow::Result;
pub use error::*;

use completions::ChatCompletions;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client as ReqwestClient, ClientBuilder};
use response::{BalanceResp, ModelResp};

#[derive(Clone)]
/// A client for interacting with the DeepSeek API.
///
/// # Example
///
/// ```no_run
/// #[tokio::main]
/// async fn main() {
///     use deepseek_api::Client;
///
///     let api_key = "your_api_key";
///     let client = Client::new(api_key);
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
    client: ReqwestClient,
    host: &'static str,
}

impl Client {
    /// Creates a new `Client` instance with the provided API key.
    ///
    /// This method initializes the client with the necessary headers, including
    /// the authorization header with the provided API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string slice that holds the API key for authorization.
    ///
    /// # Returns
    ///
    /// A new instance of the `Client` struct.
    ///
    /// # Panics
    ///
    /// This function will panic if the `HeaderValue` cannot be created from the
    /// provided API key or if the `Client` cannot be built.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use deepseek_api::Client;
    ///
    /// let client = Client::new("your_api_key");
    /// ```
    pub fn new(api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        let bearer = format!("Bearer {}", api_key);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&bearer).expect("bearer"),
        );

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("Client::new()");

        Client {
            client,
            host: "https://api.deepseek.com",
        }
    }

    pub fn chat(&self) -> ChatCompletions {
        ChatCompletions {
            client: self.client.clone(),
            host: self.host,
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
    ///     use deepseek_api::Client;
    ///
    ///     let client = Client::new("your_api_key");
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
    ///     use deepseek_api::Client;
    ///
    ///     let client = Client::new("your_api_key");
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
