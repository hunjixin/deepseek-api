use serde::{de::DeserializeOwned, ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::{
    completions::ChatCompletions,
    request::{
        FrequencyPenalty, MaxToken, MessageRequest, PresencePenalty, ResponseFormat, ResponseType,
        Stop, StreamOptions, Temperature, ToolChoice, ToolObject, TopLogprobs, TopP,
        UserMessageRequest,
    },
    response::{
        AssistantMessage, ChatCompletion, ChatCompletionStream, ChatResponse, JSONChoiceStream,
        ModelType, TextChoiceStream,
    },
};

use anyhow::{Ok, Result};

pub trait RequestBuilder: Sized + Send {
    type Request: Serialize + Send;
    type Response: DeserializeOwned + Send + 'static;
    type Item: DeserializeOwned + Send + 'static;

    fn is_beta(&self) -> bool;
    fn is_stream(&self) -> bool;
    fn build(self) -> Self::Request;

    cfg_if::cfg_if! {
        if #[cfg(feature = "is_sync")] {
            fn do_request(self, client: &ChatCompletions) ->  Result<ChatResponse<Self::Response, Self::Item>>  {
                client.create(self)
            }
        } else {
            fn do_request(self, client: &ChatCompletions) ->  impl std::future::Future<Output = Result<ChatResponse<Self::Response, Self::Item>>> + Send {async {
                    client.create(self).await
            }}
        }
    }
}

/// Represents a request for completions.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CompletionsRequest {
    pub messages: Vec<MessageRequest>,
    pub model: ModelType,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<MaxToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    // ignore when model is deepseek-reasoner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<TopP>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<PresencePenalty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<FrequencyPenalty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<TopLogprobs>,
}

impl Serialize for CompletionsRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CompletionsRequest", 12)?;

        state.serialize_field("messages", &self.messages)?;
        state.serialize_field("model", &self.model)?;
        state.serialize_field("max_tokens", &self.max_tokens)?;
        state.serialize_field("response_format", &self.response_format)?;
        state.serialize_field("stop", &self.stop)?;
        state.serialize_field("stream", &self.stream)?;
        state.serialize_field("stream_options", &self.stream_options)?;
        state.serialize_field("tools", &self.tools)?;
        state.serialize_field("tool_choice", &self.tool_choice)?;
        state.serialize_field("prompt", &self.prompt)?;

        // Skip these fields if model is DeepSeekReasoner
        if self.model != ModelType::DeepSeekReasoner {
            state.serialize_field("temperature", &self.temperature)?;
            state.serialize_field("top_p", &self.top_p)?;
            state.serialize_field("presence_penalty", &self.presence_penalty)?;
            state.serialize_field("frequency_penalty", &self.frequency_penalty)?;
            state.serialize_field("logprobs", &self.logprobs)?;
            state.serialize_field("top_logprobs", &self.top_logprobs)?;
        }

        state.end()
    }
}

#[derive(Debug, Default)]
pub struct CompletionsRequestBuilder {
    //todo too many colone when use this type, improve it especially for message field
    beta: bool,
    messages: Vec<MessageRequest>,
    model: ModelType,

    stream: bool,
    stream_options: Option<StreamOptions>,

    max_tokens: Option<MaxToken>,
    response_format: Option<ResponseFormat>,
    stop: Option<Stop>,
    tools: Option<Vec<ToolObject>>,
    tool_choice: Option<ToolChoice>,
    prompt: String,
    temperature: Option<Temperature>,
    top_p: Option<TopP>,
    presence_penalty: Option<PresencePenalty>,
    frequency_penalty: Option<FrequencyPenalty>,
    logprobs: Option<bool>,
    top_logprobs: Option<TopLogprobs>,
}

impl CompletionsRequestBuilder {
    pub fn new(messages: Vec<MessageRequest>) -> Self {
        Self {
            messages,
            model: ModelType::DeepSeekChat,
            prompt: String::new(),
            ..Default::default()
        }
    }
    pub fn use_model(mut self, model: ModelType) -> Self {
        self.model = model;
        self
    }

    //https://api-docs.deepseek.com/guides/fim_completion
    pub fn append_fim_message(self, _prompt: &str, _suffix: &str) -> Self {
        todo!("Not enough detail in document")
    }

    // https://api-docs.deepseek.com/zh-cn/guides/chat_prefix_completion
    pub fn append_prefix_message(mut self, msg: &str) -> Self {
        self.messages.push(MessageRequest::Assistant(
            AssistantMessage::new(msg).set_prefix(msg),
        ));
        self
    }

    pub fn append_user_message(mut self, msg: &str) -> Self {
        self.messages
            .push(MessageRequest::User(UserMessageRequest::new(msg)));
        self
    }

    pub fn max_tokens(mut self, value: u32) -> Result<Self> {
        self.max_tokens = Some(MaxToken::new(value)?);
        Ok(self)
    }

    pub fn use_beta(mut self, value: bool) -> Self {
        self.beta = value;
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = value;
        self
    }

    pub fn stream_options(mut self, value: StreamOptions) -> Self {
        self.stream_options = Some(value);
        self
    }

    pub fn response_format(mut self, value: ResponseType) -> Self {
        self.response_format = Some(ResponseFormat { resp_type: value });
        self
    }

    pub fn stop(mut self, value: Stop) -> Self {
        self.stop = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<ToolObject>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: ToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn prompt(mut self, value: String) -> Self {
        self.prompt = value;
        self
    }

    pub fn temperature(mut self, value: u32) -> Result<Self> {
        self.temperature = Some(Temperature::new(value)?);
        Ok(self)
    }

    pub fn top_p(mut self, value: f32) -> Result<Self> {
        self.top_p = Some(TopP::new(value)?);
        Ok(self)
    }

    pub fn presence_penalty(mut self, value: f32) -> Result<Self> {
        self.presence_penalty = Some(PresencePenalty::new(value)?);
        Ok(self)
    }

    pub fn frequency_penalty(mut self, value: f32) -> Result<Self> {
        self.frequency_penalty = Some(FrequencyPenalty::new(value)?);
        Ok(self)
    }

    pub fn logprobs(mut self, value: bool) -> Self {
        self.logprobs = Some(value);
        self
    }

    pub fn top_logprobs(mut self, value: u32) -> Result<Self> {
        self.top_logprobs = Some(TopLogprobs::new(value)?);
        Ok(self)
    }
}

impl RequestBuilder for CompletionsRequestBuilder {
    type Request = CompletionsRequest;
    type Response = ChatCompletion;
    type Item = ChatCompletionStream<JSONChoiceStream>;

    fn is_beta(&self) -> bool {
        self.beta
    }

    fn is_stream(&self) -> bool {
        self.stream
    }

    fn build(self) -> CompletionsRequest {
        CompletionsRequest {
            messages: self.messages,
            model: self.model,
            max_tokens: self.max_tokens,
            response_format: self.response_format,
            stop: self.stop,
            stream: self.stream,
            stream_options: self.stream_options,
            tools: self.tools,
            tool_choice: self.tool_choice,
            prompt: self.prompt,
            temperature: self.temperature,
            top_p: self.top_p,
            presence_penalty: self.presence_penalty,
            frequency_penalty: self.frequency_penalty,
            logprobs: self.logprobs,
            top_logprobs: self.top_logprobs,
        }
    }
}

/// Represents a request for completions.
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct FMICompletionsRequest {
    pub model: ModelType,
    pub prompt: String,
    pub echo: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<FrequencyPenalty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<MaxToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<PresencePenalty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    pub suffix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<TopP>,
}

#[derive(Debug, Default)]
pub struct FMICompletionsRequestBuilder {
    model: ModelType,
    prompt: String,
    echo: bool,
    frequency_penalty: Option<FrequencyPenalty>,
    logprobs: Option<bool>,
    max_tokens: Option<MaxToken>,
    presence_penalty: Option<PresencePenalty>,
    stop: Option<Stop>,
    stream: bool,
    stream_options: Option<StreamOptions>,
    suffix: String,
    temperature: Option<Temperature>,
    top_p: Option<TopP>,
}

impl FMICompletionsRequestBuilder {
    pub fn new(prompt: &str, suffix: &str) -> Self {
        Self {
            //fim only support deepseek-chat model
            model: ModelType::DeepSeekChat,
            prompt: prompt.to_string(),
            suffix: suffix.to_string(),
            echo: false,
            stream: false,
            ..Default::default()
        }
    }

    pub fn echo(mut self, value: bool) -> Self {
        self.echo = value;
        self
    }

    pub fn frequency_penalty(mut self, value: f32) -> Result<Self> {
        self.frequency_penalty = Some(FrequencyPenalty::new(value)?);
        Ok(self)
    }

    pub fn logprobs(mut self, value: bool) -> Self {
        self.logprobs = Some(value);
        self
    }

    pub fn max_tokens(mut self, value: u32) -> Result<Self> {
        self.max_tokens = Some(MaxToken::new(value)?);
        Ok(self)
    }

    pub fn presence_penalty(mut self, value: f32) -> Result<Self> {
        self.presence_penalty = Some(PresencePenalty::new(value)?);
        Ok(self)
    }

    pub fn stop(mut self, value: Stop) -> Self {
        self.stop = Some(value);
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = value;
        self
    }

    pub fn stream_options(mut self, value: StreamOptions) -> Self {
        self.stream_options = Some(value);
        self
    }

    pub fn temperature(mut self, value: u32) -> Result<Self> {
        self.temperature = Some(Temperature::new(value)?);
        Ok(self)
    }

    pub fn top_p(mut self, value: f32) -> Result<Self> {
        self.top_p = Some(TopP::new(value)?);
        Ok(self)
    }
}

impl RequestBuilder for FMICompletionsRequestBuilder {
    type Request = FMICompletionsRequest;
    type Response = ChatCompletion;
    type Item = ChatCompletionStream<TextChoiceStream>;

    fn is_beta(&self) -> bool {
        true
    }

    fn is_stream(&self) -> bool {
        self.stream
    }

    fn build(self) -> FMICompletionsRequest {
        FMICompletionsRequest {
            model: self.model,
            prompt: self.prompt,
            echo: self.echo,
            frequency_penalty: self.frequency_penalty,
            logprobs: self.logprobs,
            max_tokens: self.max_tokens,
            presence_penalty: self.presence_penalty,
            stop: self.stop,
            stream: self.stream,
            stream_options: self.stream_options,
            suffix: self.suffix,
            temperature: self.temperature,
            top_p: self.top_p,
        }
    }
}
