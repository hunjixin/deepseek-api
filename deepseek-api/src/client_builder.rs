use anyhow::{Ok, Result};
use std::env;
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
///     .with_timeout(30)
///     .build()
///     .expect("Failed to build client");
/// ```
pub struct DeepSeekClientBuilder {
    api_key: String,
    timeout: Option<u64>,
    host: String,
}

impl Default for DeepSeekClientBuilder {
    /// Creates a new `DeepSeekClientBuilder` instance with default settings.
    ///
    /// This method attempts to retrieve the API key from the environment variable
    /// `DEEPSEEK_API_KEY`. If the environment variable is not set, it uses a empty string as
    /// default API key. The `timeout` is set to `None`, indicating
    /// no default timeout, and the `host` is set to the default API URL `"https://api.deepseek.com"`.
    ///
    /// # Returns
    /// A new instance of `DeepSeekClientBuilder` with default values.
    fn default() -> Self {
        let api_key = env::var("DEEPSEEK_API_KEY").unwrap_or_else(|_| String::from(""));

        DeepSeekClientBuilder {
            api_key,
            timeout: None,
            host: String::from("https://api.deepseek.com"),
        }
    }
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
    ///     .with_timeout(30);
    /// ```
    pub fn with_timeout(mut self, duration: u64) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// Sets the host url for the client.
    ///
    /// # Arguments
    ///
    /// * `host` - A `string` value representing the host url.
    ///
    /// # Returns
    ///
    /// The `DeepSeekClientBuilder` instance with the host configured.
    /// ```ignore
    /// let builder = DeepSeekClientBuilder::new("your_api_key".to_string())
    ///     .with_host("https://api.deepseek.com");
    /// ```
    pub fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    /// Sets the api key for the client.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A `string` value representing the api key.
    ///
    /// # Returns
    ///
    /// The `DeepSeekClientBuilder` instance with the api key configured.
    /// ```ignore
    /// let builder = DeepSeekClientBuilder::new("your_api_key".to_string())
    ///     .with_api_key("your_api_key");
    /// ```
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
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
    ///     .with_timeout(30)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_deep_seek_client_builder_from_env_var() {
        env::set_var("DEEPSEEK_API_KEY", "test_api_key");

        let builder = DeepSeekClientBuilder::default().with_timeout(15);

        assert_eq!(builder.host, "https://api.deepseek.com");
        assert_eq!(builder.timeout, Some(15));
        assert_eq!(builder.api_key, "test_api_key");

        assert!(builder.build().is_ok());
    }

    #[test]
    fn test_deep_seek_client_override_options() {
        // Build the client using the builder with a provided API key
        let builder = DeepSeekClientBuilder::new("test_api_key".to_string())
            .with_host("http://override.com")
            .with_api_key("another_test_api_keyu");

        assert_eq!(builder.host, "http://override.com");
        assert_eq!(builder.api_key, "another_test_api_keyu");

        assert!(builder.build().is_ok());
    }

    #[test]
    fn test_deep_seek_client_builder_from_new_function() {
        // Build the client using the builder with a provided API key
        let builder = DeepSeekClientBuilder::new("test_api_key".to_string()).with_timeout(20);

        assert_eq!(builder.host, "https://api.deepseek.com");
        assert_eq!(builder.timeout, Some(20));
        assert_eq!(builder.api_key, "test_api_key");

        assert!(builder.build().is_ok());
    }
}
