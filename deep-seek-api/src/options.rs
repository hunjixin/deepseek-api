use schemars::schema::SchemaObject;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use crate::{ModelType, Message};

/// Represents a frequency penalty with a value between -2 and 2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrequencyPenalty(pub f32);

impl FrequencyPenalty {
    /// Creates a new `FrequencyPenalty` instance.
    ///
    /// # Arguments
    ///
    /// * `v` - A float value representing the frequency penalty.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not between -2 and 2.
    pub fn new(v: f32) -> Result<Self> {
        if v < -2.0 || v > 2.0 {
            return Err(anyhow!("Frequency penalty value must be between -2 and 2.".to_string()));
        }
        Ok(FrequencyPenalty(v))
    }
}

impl Default for FrequencyPenalty {
    /// Returns the default value for `FrequencyPenalty`, which is 0.0.
    fn default() -> Self {
        FrequencyPenalty(0.0)
    }
}

/// Represents a presence penalty with a value between -2 and 2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PresencePenalty(pub f32);

impl PresencePenalty {
    /// Creates a new `PresencePenalty` instance.
    ///
    /// # Arguments
    ///
    /// * `v` - A float value representing the presence penalty.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not between -2 and 2.
    pub fn new(v: f32) -> Result<Self> {
        if v < -2.0 || v > 2.0 {
            return Err(anyhow!("Presence penalty value must be between -2 and 2.".to_string()));
        }
        Ok(PresencePenalty(v))
    }
}

impl Default for PresencePenalty {
    /// Returns the default value for `PresencePenalty`, which is 0.0.
    fn default() -> Self {
        PresencePenalty(0.0)
    }
}

/// Represents the type of response.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ResponseType {
    #[serde(rename = "json_object")]
    Json,
    #[serde(rename = "text")]
    Text,
}

/// Represents the format of the response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseForamt {
    #[serde(rename = "type")]
    pub resp_type: ResponseType,
}

impl ResponseForamt {
    /// Creates a new `ResponseForamt` instance.
    ///
    /// # Arguments
    ///
    /// * `rt` - The type of response.
    pub fn new(rt: ResponseType) -> Self {
        ResponseForamt { resp_type: rt }
    }
}

/// Represents the maximum number of tokens with a value between 1 and 8192.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaxToken(pub u32);

impl MaxToken {
    /// Creates a new `MaxToken` instance.
    ///
    /// # Arguments
    ///
    /// * `v` - An unsigned integer representing the maximum number of tokens.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not between 1 and 8192.
    pub fn new(v: u32) -> Result<Self> {
        if v < 1 || v > 8192 {
            return Err(anyhow!("Max token must be between 1 and 8192.".to_string()));
        }
        Ok(MaxToken(v))
    }
}

impl Default for MaxToken {
    /// Returns the default value for `MaxToken`, which is 4096.
    fn default() -> Self {
        MaxToken(4096)
    }
}

/// Represents the stopping criteria for the completion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stop {
    Single(String),
    Multiple(Vec<String>),
}

/// Represents the options for streaming responses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamOptions {
    pub include_usage: bool,
}

impl StreamOptions {
    /// Creates a new `StreamOptions` instance.
    ///
    /// # Arguments
    ///
    /// * `include_usage` - A boolean indicating whether to include usage information.
    pub fn new(include_usage: bool) -> Self {
        StreamOptions { include_usage }
    }
}

/// Represents the temperature with a value between 0 and 2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Temperature(pub u32);

impl Temperature {
    /// Creates a new `Temperature` instance.
    ///
    /// # Arguments
    ///
    /// * `v` - An unsigned integer representing the temperature.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not between 0 and 2.
    pub fn new(v: u32) -> Result<Self> {
        if v > 2 {
            return Err(anyhow!("Temperature must be between 0 and 2.".to_string()));
        }
        Ok(Temperature(v))
    }
}

impl Default for Temperature {
    /// Returns the default value for `Temperature`, which is 1.
    fn default() -> Self {
        Temperature(1)
    }
}

/// Represents the top-p value with a value between 0.0 and 1.0.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopP(pub f32);

impl TopP {
    /// Creates a new `TopP` instance.
    ///
    /// # Arguments
    ///
    /// * `v` - A float value representing the top-p value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not between 0.0 and 1.0.
    pub fn new(v: f32) -> Result<Self> {
        if v < 0.0 || v > 1.0 {
            return Err(anyhow!("TopP value must be between 0and 2.".to_string()));
        }
        Ok(TopP(v))
    }
}

impl Default for TopP {
    /// Returns the default value for `TopP`, which is 1.0.
    fn default() -> Self {
        TopP(1.0)
    }
}

/// Represents the type of tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolType {
    #[serde(rename = "function")]
    Function,
}

/// Represents a function with a description, name, and parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub description: String,
    pub name: String,
    pub parameters: SchemaObject,
}

/// Represents a tool object with a type and function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolObject {
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    pub function: Function,
}

/// Represents the choice of chat completion tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionToolChoice {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}

/// Represents a function choice with a name.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionChoice {
    pub name: String,
}

/// Represents the choice of named chat completion tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionNamedToolChoice {
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    pub function: FunctionChoice,
}

/// Represents the choice of tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolChoice {
    ChatCompletion(ChatCompletionToolChoice),
    ChatCompletionNamed(ChatCompletionNamedToolChoice),
}

/// Represents the top log probabilities with a value between 0 and 20.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopLogprobs(pub u32);

impl TopLogprobs {
    /// Creates a new `TopLogprobs` instance.
    ///
    /// # Arguments
    ///
    /// * `v` - An unsigned integer representing the top log probabilities.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not between 0 and 20.
    pub fn new(v: u32) -> Result<Self> {
        if v > 20 {
            return Err(anyhow!("Top log probs must be between 0 and 20.".to_string()));
        }
        Ok(TopLogprobs(v))
    }
}

impl Default for TopLogprobs {
    /// Returns the default value for `TopLogprobs`, which is 0.
    fn default() -> Self {
        TopLogprobs(0)
    }
}

/// Represents a message request with different roles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "role")]
pub enum MessageRequest {
    #[serde(rename = "system")]
    System(SystemMessageRequest),
    #[serde(rename = "user")]
    User(UserMessageRequest),
    #[serde(rename = "assistant")]
    Assistant(AssistantMessageRequest),
    #[serde(rename = "tool")]
    Tool(ToolMessageRequest)
}

impl MessageRequest {
    /// Creates a `MessageRequest` instance from a `Message`.
    ///
    /// # Arguments
    ///
    /// * `resp_message` - A reference to a `Message`.
    ///
    /// # Errors
    ///
    /// Returns an error if the message role is invalid.
    pub fn from_message(resp_message: &Message) -> Result<Self> {
        match resp_message.role.as_str() {
            "system" => Ok(MessageRequest::System(SystemMessageRequest {
                content: resp_message.content.clone(),
                name: None,
            })),
            "user" => Ok(MessageRequest::User(UserMessageRequest {
                content: resp_message.content.clone(),
                name: None,
            })),
            "assistant" => {
                let request = match resp_message.reasoning_content.clone() {
                    Some(reasoning_content) => {
                        AssistantMessageRequest::new(resp_message.content.as_str())
                            .set_reasoning_content(reasoning_content.as_str())
                    },
                    None => AssistantMessageRequest::new(resp_message.content.as_str())
                };
                Ok(MessageRequest::Assistant(request))
            },
            "tool" => Ok(MessageRequest::Tool(ToolMessageRequest {
                content: resp_message.content.clone(),
                tool_call_id: "".to_string(),  //todo how to get tool_call_id ?
            })),
            _ => Err(anyhow!("Invalid message role.".to_string())),
        }
    }
}

/// Represents a system message request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemMessageRequest {
    pub content: String,
    pub name: Option<String>,
}

impl SystemMessageRequest {
    /// Creates a new `SystemMessageRequest` instance.
    ///
    /// # Arguments
    ///
    /// * `msg` - A string slice representing the message content.
    pub fn new(msg: &str) -> Self {
        SystemMessageRequest {
            content: msg.to_string(),
            name: None,
        }
    }

    /// Creates a new `SystemMessageRequest` instance with a name.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice representing the name.
    /// * `msg` - A string slice representing the message content.
    pub fn new_with_name(name: &str, msg: &str) -> Self {
        SystemMessageRequest {
            content: msg.to_string(),
            name: Some(name.to_string()),
        }
    }
}

/// Represents a user message request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserMessageRequest {
    pub content: String,
    pub name: Option<String>,
}

impl UserMessageRequest {
    /// Creates a new `UserMessageRequest` instance.
    ///
    /// # Arguments
    ///
    /// * `msg` - A string slice representing the message content.
    pub fn new(msg: &str) -> Self {
        UserMessageRequest {
            content: msg.to_string(),
            name: None,
        }
    }

    /// Creates a new `UserMessageRequest` instance with a name.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice representing the name.
    /// * `msg` - A string slice representing the message content.
    pub fn new_with_name(name: &str, msg: &str) -> Self {
        UserMessageRequest {
            content: msg.to_string(),
            name: Some(name.to_string()),
        }
    }
}

/// Represents an assistant message request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssistantMessageRequest {
    pub content: String,
    pub name: Option<String>,
    pub prefix: bool,
    pub reasoning_content: Option<String>
}

impl AssistantMessageRequest {
    /// Creates a new `AssistantMessageRequest` instance.
    ///
    /// # Arguments
    ///
    /// * `msg` - A string slice representing the message content.
    pub fn new(msg: &str) -> Self {
        AssistantMessageRequest {
            content: msg.to_string(),
            name: None,
            prefix:false,
            reasoning_content:None,
        }
    }

    /// Creates a new `AssistantMessageRequest` instance with a name.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice representing the name.
    /// * `msg` - A string slice representing the message content.
    pub fn new_with_name(name: &str, msg: &str) -> Self {
        AssistantMessageRequest {
            content: msg.to_string(),
            name: Some(name.to_string()),
            prefix:false,
            reasoning_content:None
        }
    }

    /// Sets the reasoning content for the `AssistantMessageRequest`.
    ///
    /// # Arguments
    ///
    /// * `content` - A string slice representing the reasoning content.
    ///
    /// # Returns
    ///
    /// Returns the updated `AssistantMessageRequest` instance.
    pub fn set_reasoning_content(mut self, content: &str) ->Self{
        self.prefix = true;
        self.reasoning_content = Some(content.to_string());
        self
    }
}

/// Represents a tool message request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolMessageRequest {
    pub content: String,
    pub tool_call_id: String,
}

impl ToolMessageRequest {
    /// Creates a new `ToolMessageRequest` instance.
    ///
    /// # Arguments
    ///
    /// * `msg` - A string slice representing the message content.
    /// * `tool_call_id` - A string slice representing the tool call ID.
    pub fn new(msg: &str, tool_call_id: &str) -> Self {
        ToolMessageRequest {
            content: msg.to_string(),
            tool_call_id: tool_call_id.to_string(),
        }
    }
}

/// Represents a request for completions.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletionsRequest {
    pub messages: Vec<MessageRequest>,
    pub model: ModelType,
    pub frequency_penalty: Option<FrequencyPenalty>,
    pub max_tokens: Option<MaxToken>,
    pub presence_penalty: Option<PresencePenalty>,
    pub response_format: Option<ResponseForamt>,
    pub stop: Option<Stop>,
    pub stream: bool,
    pub stream_options: Option<StreamOptions>,
    pub temperature: Option<Temperature>,
    pub top_p: Option<TopP>,
    pub tools: Option<Vec<ToolObject>>,
    pub tool_choice: Option<ToolChoice>,
    pub logprobs: Option<bool>,
    pub prompt: String, //nochange not in document
    pub top_logprobs: Option<TopLogprobs>,
}
