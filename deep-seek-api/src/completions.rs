use crate::{
    error::ToApiError,
    request::{
        CompletionsRequestBuilder, FMICompletionsRequestBuilder, MessageRequest, RequestBuilder,
    },
    response::ModelType,
};
use anyhow::Result;
use reqwest::{ClientBuilder, Client as ReqwestClient};
pub struct Completions {
    pub(crate) client: ReqwestClient,
    pub(crate) host: &'static str,
    pub(crate) model: ModelType,
}

impl Completions {
    pub fn set_model(mut self, model: ModelType) -> Self {
        self.model = model;
        self
    }

    pub fn chat_builder(&self, messages: Vec<MessageRequest>) -> CompletionsRequestBuilder {
        CompletionsRequestBuilder::new(messages, self.model.clone())
    }

    pub fn fim_builder(&self, prompt: &str, suffix: &str) -> FMICompletionsRequestBuilder {
        FMICompletionsRequestBuilder::new(self.model.clone(), prompt, suffix)
    }

    pub async fn create<Builder>(&mut self, request_builder: Builder) -> Result<Builder::Response>
    where
        Builder: RequestBuilder,
    {
        let host = if request_builder.is_beta() {
            self.host.to_owned() + "/beta/completions"
        } else {
            self.host.to_owned() + "/chat/completions"
        };

        let request = request_builder.build();

        Ok(self
            .client
            .post(&host)
            .json(&request)
            .send()
            .await?
            .to_api_err()
            .await?
            .json()
            .await?)
    }
}
