use anyhow::{Ok, Result};
cfg_if::cfg_if! {
    if #[cfg(feature = "is_sync")] {
        use reqwest::blocking::ClientBuilder as ReqwestClientBuilder;
    } else {
        use reqwest::ClientBuilder as ReqwestClientBuilder;
    }
}
use crate::DeepSeekClient;
use reqwest::header::HeaderMap;
use std::time::Duration;

/// A builder for constructing a `DeepSeekClient` instance with customizable options.
///
/// The `DeepSeekClientBuilder` allows you to configure the API key, timeout, and host
/// for the `DeepSeekClient` before building it.
///
/// # Examples
///
/// ```ignore
/// let client = DeepSeekClientBuilder::new("your_api_key".to_string())
///     .timeout(30)
///     .build()
///     .expect("Failed to build client");
/// ```
pub struct DeepSeekClientBuilder {
    api_key: String,
    timeout: Option<u64>,
    host: String,
}

impl DeepSeekClientBuilder {
    /// Creates a new `DeepSeekClientBuilder` with the specified API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A `String` containing the API key to be used for authentication.
    ///
    /// # Returns
    ///
    /// A new instance of `DeepSeekClientBuilder` with default settings.
    ///
    /// The default host is set to `"https://api.deepseek.com"`, and no timeout is configured.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            timeout: None,
            host: "https://api.deepseek.com".to_string(),
        }
    }

    /// Sets the timeout duration for the client.
    ///
    /// # Arguments
    ///
    /// * `duration` - A `u64` value representing the timeout duration in seconds.
    ///
    /// # Returns
    ///
    /// The `DeepSeekClientBuilder` instance with the timeout configured.
    /// ```ignore
    /// let builder = DeepSeekClientBuilder::new("your_api_key".to_string())
    ///     .timeout(30);
    /// ```
    pub fn timeout(mut self, duration: u64) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// Builds the `Client` instance using the configured options.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed `Client` on success, or an error
    /// if the client could not be built.
    ///
    /// # Errors
    ///
    /// This method will return an error if the underlying `reqwest::blocking::DeepSeekClientBuilder`
    /// fails to build the client.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let client = DeepSeekClientBuilder::new("your_api_key".to_string())
    ///     .timeout(30)
    ///     .build()
    ///     .expect("Failed to build client");
    /// ```
    pub fn build(self) -> Result<DeepSeekClient> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", self.api_key).parse()?);

        let client_builder = ReqwestClientBuilder::new().default_headers(headers);
        let client_builder = if let Some(secs) = self.timeout {
            client_builder.timeout(Duration::from_secs(secs))
        } else {
            client_builder
        };

        let client = client_builder.build()?;
        Ok(DeepSeekClient {
            client,
            host: self.host,
        })
    }
}
