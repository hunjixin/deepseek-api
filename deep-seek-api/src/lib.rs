#![feature(trait_alias)]

pub mod completions;
mod error;

pub mod json_stream;
pub mod request;
pub mod response;

use anyhow::Result;
pub use error::*;

use completions::Completions;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client as ReqwestClient, ClientBuilder};
use response::{BalanceResp, ModelResp, ModelType};

#[derive(Clone)]
pub struct Client {
    client: ReqwestClient,
    host: &'static str,
}

impl Client {
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

    pub fn completions(&self) -> Completions {
        Completions {
            client: self.client.clone(),
            host: self.host,
            model: ModelType::DeepSeekChat,
        }
    }

    pub async fn models(&self) -> Result<ModelResp> {
        Ok(self
            .client
            .get(self.host.to_owned() + "/models")
            .send()
            .await?
            .json()
            .await?)
    }

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
