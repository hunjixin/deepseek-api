use serde::{de::DeserializeOwned, ser::SerializeStruct, Serialize, Serializer};

use crate::{
    request::{
        FrequencyPenalty, MaxToken, MessageRequest, PresencePenalty, ResponseFormat, ResponseType,
        Stop, StreamOptions, Temperature, ToolChoice, ToolObject, TopLogprobs, TopP,
    },
    response::{
        ChatCompletion, ChatCompletionStream, ChatResponse, JSONChoiceStream, ModelType,
        TextChoiceStream,
    },
    DeepSeekClient,
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
            fn do_request(self, client: &DeepSeekClient) ->  Result<ChatResponse<Self::Response, Self::Item>>  {
                client.send_completion_request(self)
            }
        } else {
            fn do_request(self, client: &DeepSeekClient) ->  impl std::future::Future<Output = Result<ChatResponse<Self::Response, Self::Item>>> + Send {async {
                client.send_completion_request(self).await
            }}
        }
    }
}

/// Represents a request for completions.
#[derive(Debug, Default, Clone)]
pub struct CompletionsRequest<'a> {
    pub messages: &'a [MessageRequest],
    pub model: ModelType,
    pub max_tokens: Option<MaxToken>,
    pub response_format: Option<ResponseFormat>,
    pub stop: Option<Stop>,
    pub stream: bool,
    pub stream_options: Option<StreamOptions>,
    pub tools: Option<&'a [ToolObject]>,
    pub tool_choice: Option<ToolChoice>,

    // ignore when model is deepseek-reasoner
    pub temperature: Option<Temperature>,
    pub top_p: Option<TopP>,
    pub presence_penalty: Option<PresencePenalty>,
    pub frequency_penalty: Option<FrequencyPenalty>,
    pub logprobs: Option<bool>,
    pub top_logprobs: Option<TopLogprobs>,
}

impl Serialize for CompletionsRequest<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CompletionsRequest", 12)?;

        state.serialize_field("messages", &self.messages)?;
        state.serialize_field("model", &self.model)?;

        if let Some(max_tokens) = &self.max_tokens {
            state.serialize_field("max_tokens", max_tokens)?;
        }
        if let Some(response_format) = &self.response_format {
            state.serialize_field("response_format", response_format)?;
        }
        if let Some(stop) = &self.stop {
            state.serialize_field("stop", stop)?;
        }
        state.serialize_field("stream", &self.stream)?;
        if let Some(stream_options) = &self.stream_options {
            state.serialize_field("stream_options", stream_options)?;
        }
        if let Some(tools) = &self.tools {
            state.serialize_field("tools", tools)?;
        }
        if let Some(tool_choice) = &self.tool_choice {
            state.serialize_field("tool_choice", tool_choice)?;
        }

        // Skip these fields if model is DeepSeekReasoner
        if self.model != ModelType::DeepSeekReasoner {
            if let Some(temperature) = &self.temperature {
                state.serialize_field("temperature", temperature)?;
            }
            if let Some(top_p) = &self.top_p {
                state.serialize_field("top_p", top_p)?;
            }
            if let Some(presence_penalty) = &self.presence_penalty {
                state.serialize_field("presence_penalty", presence_penalty)?;
            }
            if let Some(frequency_penalty) = &self.frequency_penalty {
                state.serialize_field("frequency_penalty", frequency_penalty)?;
            }
            if let Some(logprobs) = &self.logprobs {
                state.serialize_field("logprobs", logprobs)?;
            }
            if let Some(top_logprobs) = &self.top_logprobs {
                state.serialize_field("top_logprobs", top_logprobs)?;
            }
        }

        state.end()
    }
}

#[derive(Debug, Default)]
pub struct CompletionsRequestBuilder<'a> {
    //todo too many colone when use this type, improve it especially for message field
    beta: bool,
    messages: &'a [MessageRequest],
    model: ModelType,

    stream: bool,
    stream_options: Option<StreamOptions>,

    max_tokens: Option<MaxToken>,
    response_format: Option<ResponseFormat>,
    stop: Option<Stop>,
    tools: Option<&'a [ToolObject]>,
    tool_choice: Option<ToolChoice>,
    temperature: Option<Temperature>,
    top_p: Option<TopP>,
    presence_penalty: Option<PresencePenalty>,
    frequency_penalty: Option<FrequencyPenalty>,
    logprobs: Option<bool>,
    top_logprobs: Option<TopLogprobs>,
}

impl<'a> CompletionsRequestBuilder<'a> {
    pub fn new(messages: &'a [MessageRequest]) -> Self {
        Self {
            messages,
            model: ModelType::DeepSeekChat,
            ..Default::default()
        }
    }
    pub fn use_model(mut self, model: ModelType) -> Self {
        self.model = model;
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

    pub fn tools(mut self, value: &'a [ToolObject]) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: ToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn temperature(mut self, value: f32) -> Result<Self> {
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

impl<'a> RequestBuilder for CompletionsRequestBuilder<'a> {
    type Request = CompletionsRequest<'a>;
    type Response = ChatCompletion;
    type Item = ChatCompletionStream<JSONChoiceStream>;

    fn is_beta(&self) -> bool {
        self.beta
    }

    fn is_stream(&self) -> bool {
        self.stream
    }

    fn build(self) -> CompletionsRequest<'a> {
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
    pub suffix: String,

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

    pub fn temperature(mut self, value: f32) -> Result<Self> {
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
