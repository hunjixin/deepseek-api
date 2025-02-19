use crate::{
    options::{CompletionsRequest, MessageRequest},
    ChatCompletion, Message, ModelType,
};
use anyhow::Result;
use reqwest::Client as HttpClient;
pub struct Completions {
    pub(crate) client: HttpClient,
    pub(crate) host: &'static str,
    pub(crate) messages: Vec<MessageRequest>,
}

impl Completions {
    pub async fn talk(&mut self, msg: &str) -> Result<Vec<Message>> {
        let mut message = self.messages.clone();
        message.push(MessageRequest::new(msg));
        let body_params = CompletionsRequest {
            messages: message,
            model: ModelType::DeepSeekChat,
            ..Default::default()
        };
        {
            let texts = self
            .client
            .post(self.host.to_owned() + "/completions")
            .json(&body_params)
            .send()
            .await?
            .text()
            .await?;
            println!("text   {}", texts);
            println!("xxxxxxx")
        }
        let results: ChatCompletion = self
            .client
            .post(self.host.to_owned() + "/completions")
            .json(&body_params)
            .send()
            .await?
            .json()
            .await?;

        let resp_message = results
            .choices
            .iter()
            .map(|choice| {
                self.messages
                    .push(MessageRequest::new(&choice.message.content));
                choice.message.clone()
            })
            .collect();

        Ok(resp_message)
    }
}
