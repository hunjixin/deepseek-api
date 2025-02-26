use crate::{
    error::ToApiError,
    request::{
        AssistantMessageRequest, CompletionsRequestBuilder, FMICompletionsRequestBuilder,
        InToRequest, MessageRequest,
    },
    response::{ChatCompletion, Message, ModelType},
};
use anyhow::Result;
use reqwest::Client as HttpClient;

pub struct Completions {
    pub(crate) client: HttpClient,
    pub(crate) host: &'static str,
    pub(crate) model: ModelType,
    pub(crate) messages: Vec<MessageRequest>,
}

impl Completions {
    pub fn set_model(mut self, model: ModelType) -> Self {
        self.model = model;
        self
    }

    pub fn chat_builder(&self) -> CompletionsRequestBuilder {
        CompletionsRequestBuilder::new(self.messages.clone(), self.model.clone())
    }

    pub fn fim_builder(&self, prompt: &str, suffix: &str) -> FMICompletionsRequestBuilder {
        FMICompletionsRequestBuilder::new(self.model.clone(), prompt, suffix)
    }

    pub async fn create<Builder>(&mut self, request_builder: Builder) -> Result<Vec<String>>
    where
        Builder: InToRequest,
    {
        let host = if request_builder.is_beta() {
            self.host.to_owned() + "/beta/completions"
        } else {
            self.host.to_owned() + "/chat/completions"
        };

        let request = request_builder.build();

        let body = serde_json::to_string(&request)?;
        println!("request: {}", body);
        let results: ChatCompletion = self
            .client
            .post(&host)
            .json(&request)
            .send()
            .await?
            .to_api_err()
            .await?
            .json()
            .await?;

        let resp_message = results
            .choices
            .iter()
            .map(|choice| {
                if let Some(msg) = &choice.message {
                    if self.model == ModelType::DeepSeekChat {
                        self.messages
                            .push(MessageRequest::from_message(msg).expect("Unexpected message"));
                    } else {
                        self.messages.push(MessageRequest::Assistant(
                            AssistantMessageRequest::new(&msg.content),
                        ));
                    }
                    msg.content.clone()
                } else {
                    choice.text.clone().unwrap()
                }
            })
            .collect();

        Ok(resp_message)
    }
}
