use crate::{
    error::ToApiError,
    request::{
        AssistantMessageRequest, CompletionsRequest, CompletionsRequestBuilder, MessageRequest,
    },
    ChatCompletion, Message, ModelType,
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

    pub fn request_builder(&self) -> CompletionsRequestBuilder {
        CompletionsRequestBuilder::new(self.messages.clone(), self.model.clone())
    }

    pub async fn create(&mut self, request: &CompletionsRequest) -> Result<Vec<Message>> {
        #[cfg(feature = "beta")]
        let host = self.host.to_owned() + "/beta/completions";
        #[cfg(not(feature = "beta"))]
        let host = self.host.to_owned() + "/completions";

        let results: ChatCompletion = self
            .client
            .post(&host)
            .json(request)
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
                if self.model == ModelType::DeepSeekChat {
                    self.messages.push(
                        MessageRequest::from_message(&choice.message).expect("Unexpected message"),
                    );
                } else {
                    self.messages
                        .push(MessageRequest::Assistant(AssistantMessageRequest::new(
                            &choice.message.content,
                        )));
                }
                choice.message.clone()
            })
            .collect();

        Ok(resp_message)
    }
}
