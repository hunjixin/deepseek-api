use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;

use crate::json_stream::JsonStream;

/// Represents different types of models available in the deep seek.
#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ModelType {
    /// Default model type for chat-based interactions.
    #[default]
    #[serde(rename = "deepseek-chat")]
    DeepSeekChat,

    /// Model type for reasoning-based interactions.
    #[serde(rename = "deepseek-reasoner")]
    DeepSeekReasoner,
}

impl ModelType {
    /// Retrieves the limit information for the model.
    ///
    /// Returns a tuple containing:
    /// - `context_len`: Maximum context length unit KB. 
    /// - `thought_chain_len`: Optional maximum thought chain length.
    /// - `output_len`: Maximum output length.
    pub fn get_limit_info(&self) -> (u32, Option<u32>, u32) {
        match self {
            ModelType::DeepSeekChat => (64, None, 8),
            ModelType::DeepSeekReasoner => (64, Some(32), 8),
        }
    }
}

impl fmt::Display for ModelType {
    /// Formats the model type into a human-readable string.
    ///
    /// The output includes context length, thought chain length (if applicable),
    /// output length, and pricing information.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (context_len, thought_chain_len, output_len) = self.get_limit_info();
        match self {
            ModelType::DeepSeekChat => {
                write!(
                    f,
                    "DeepSeekChat: Context Length = {}K, Max Output Length = {}K",
                    context_len, output_len
                )
            }
            ModelType::DeepSeekReasoner => {
                write!(
                    f,
                    "DeepSeekReasoner: Context Length = {}K, Max Thought Chain Length = {:?}K, Max Output Length = {}K",
                    context_len, thought_chain_len, output_len
                )
            }
        }
    }
}

/// Represents a model with its associated metadata.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Model {
    /// Unique identifier for the model.
    pub id: String,
    /// Type of the object.
    pub object: String,
    /// Owner of the model.
    pub owned_by: String,
}

/// Response structure containing a list of models.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModelResp {
    /// Type of the object.
    pub object: String,
    /// List of models.
    pub data: Vec<Model>,
}

/// Information about the balance of a user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BalanceInfo {
    /// Currency type.
    pub currency: String,
    /// Total balance available.
    pub total_balance: String,
    /// Granted balance.
    pub granted_balance: String,
    /// Topped up balance.
    pub topped_up_balance: String,
}

/// Response structure containing balance information.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BalanceResp {
    /// Indicates if the balance is available.
    pub is_available: bool,
    /// List of balance information.
    pub balance_infos: Vec<BalanceInfo>,
}

/// Represents a function with its name and parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    /// Name of the function.
    pub name: String,
    /// Arguments of the function, its argument .
    pub arguments: String,
}

/// Represents a tool call with its associated function.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ToolCall {
    /// Unique identifier for the tool call.
    pub id: String,
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function associated with the tool call.
    pub function: Function,
}

/// Represents a message with its content and optional reasoning content and tool calls.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct AssistantMessage {    
    /// Content of the message.
    pub content: String,
    /// Optional reasoning content.
    pub reasoning_content: Option<String>,
    /// Optional list of tool calls.
    pub tool_calls: Option<Vec<ToolCall>>,
    pub name: Option<String>,
    #[serde(default)] 
    pub prefix: bool
}


impl AssistantMessage {
    /// Creates a new `AssistantMessage` instance.
    ///
    /// # Arguments
    ///
    /// * `msg` - A string slice representing the message content.
    pub fn new(msg: &str) -> Self {
        AssistantMessage {
            content: msg.to_string(),
            name: None,
            prefix: false,
            reasoning_content: None,
            ..Default::default()
        }
    }

    /// Creates a new `AssistantMessage` instance with a name.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice representing the name.
    /// * `msg` - A string slice representing the message content.
    pub fn new_with_name(name: &str, msg: &str) -> Self {
        AssistantMessage {
            content: msg.to_string(),
            name: Some(name.to_string()),
            prefix: false,
            reasoning_content: None,
            ..Default::default()
        }
    }

    pub fn set_prefix(mut self, content: &str) -> Self {
        self.prefix = true;
        self.content = content.to_string();
        self
    }
}

/// Enum representing the reason for finishing a process.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FinishReason {
    /// Process stopped.
    #[serde(rename = "stop")]
    Stop,
    /// Process finished due to length limit.
    #[serde(rename = "length")]
    Length,
    /// Process finished due to content filter.
    #[serde(rename = "content_filter")]
    ContentFilter,
    /// Process finished due to tool calls.
    #[serde(rename = "tool_calls")]
    ToolCalls,
    /// Process finished due to insufficient system resources.
    #[serde(rename = "insufficient_system_resource")]
    InsufficientSystemResource,
}

/// Wrapper for log probability information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogProbWrap {
    /// List of log probabilities.
    pub content: Vec<LogProb>,
}

/// Represents log probability information for a token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogProb {
    /// Token string.
    pub token: String,
    /// Log probability of the token.
    pub logprob: f32,
    /// Optional bytes representation of the token.
    pub bytes: Option<Vec<u8>>,
    /// List of top log probabilities.
    pub top_logprobs: Vec<TopLogProb>,
}

/// Represents top log probability information for a token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopLogProb {
    /// Token string.
    pub token: String,
    /// Log probability of the token.
    pub logprob: f32,
    /// Optional bytes representation of the token.
    pub bytes: Option<Vec<u8>>,
}

/// Represents a choice made during a process.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Choice {
    /// Reason for finishing the process.
    pub finish_reason: FinishReason,
    /// Index of the choice.
    pub index: usize,
    /// Message associated with the choice.
    pub text: Option<String>,
    /// Message associated with the choice.
    pub message: Option<AssistantMessage>,
    /// Optional log probability information.
    pub logprobs: Option<LogProbWrap>,
}

/// Represents usage information for a process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// Number of completion tokens used.
    pub completion_tokens: u64,
    /// Number of prompt tokens used.
    pub prompt_tokens: u64,
    /// Number of prompt cache hit tokens.
    pub prompt_cache_hit_tokens: u64,
    /// Number of prompt cache miss tokens.
    pub prompt_cache_miss_tokens: u64,
    /// Total number of tokens used.
    pub total_tokens: u64,
    /// Details of completion tokens used.
    pub completion_tokens_details: Option<CompletionTokensDetails>,
}

/// Details of completion tokens used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionTokensDetails {
    /// Number of reasoning tokens used.
    pub reasoning_tokens: u64,
}

/// Represents a chat completion with its associated metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletion {
    /// Unique identifier for the chat completion.
    pub id: String,
    /// List of choices made during the chat completion.
    pub choices: Vec<Choice>,
    /// Timestamp of when the chat completion was created.
    pub created: u32,
    /// Model used for the chat completion.
    pub model: String,
    /// System fingerprint associated with the chat completion.
    pub system_fingerprint: String,
    /// Type of the object.
    pub object: String,
    pub usage: Usage,
}

/// Represents a delta change in a choice stream.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Delta {
    /// Content of the delta change.
    pub content: String,
    /// Reasoning content of the delta change.
    #[serde(default)]
    pub reasoning_content: String,
    /// Role of the delta change sender.
    #[serde(default)]
    pub role: String,
}

/// Represents a choice stream with its associated delta change.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct JSONChoiceStream {
    /// Delta change in the choice stream.
    pub delta: Delta,
    /// Reason for finishing the choice stream.
    pub finish_reason: Option<FinishReason>,
    /// Index of the choice stream.
    pub index: usize,
}

/// Represents a choice stream with its associated delta change.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextChoiceStream {
    /// Delta change in the choice stream.
    pub text: String,
    /// Reason for finishing the choice stream.
    pub finish_reason: Option<FinishReason>,
    /// Index of the choice stream.
    pub index: usize,
}

/// Represents a chat completion stream with its associated metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionStream<T> {
    /// Unique identifier for the chat completion stream.
    pub id: String,
    /// List of choice streams made during the chat completion stream.
    pub choices: Vec<T>,
    /// Timestamp of when the chat completion stream was created.
    pub created: u32,
    /// Model used for the chat completion stream.
    pub model: String,
    /// System fingerprint associated with the chat completion stream.
    pub system_fingerprint: String,
    /// Type of the object.
    pub object: String,
}

/// Represents a chat response which can either be a full response or a stream of items.
///
/// This enum is generic over the response type `RESP` and the item type `ITEM`.
///
/// # Variants
///
/// - `Full(RESP)`: Represents a complete response of type `RESP`.
/// - `Stream(JsonStream<ITEM>)`: Represents a stream of items of type `ITEM`.
///
/// # Type Parameters
///
/// - `RESP`: The type of the full response. Must implement `DeserializeOwned`.
/// - `ITEM`: The type of the items in the stream. Must implement `DeserializeOwned`.
///
/// # Methods
///
/// - `must_response(self) -> RESP`: Consumes the enum and returns the full response if it is the `Full` variant. Panics if it is the `Stream` variant.
/// - `must_stream(self) -> JsonStream<ITEM>`: Consumes the enum and returns the stream if it is the `Stream` variant. Panics if it is the `Full` variant.
pub enum ChatResponse<RESP, ITEM>
where
    RESP: DeserializeOwned,
    ITEM: DeserializeOwned,
{
    Full(RESP),
    Stream(JsonStream<ITEM>),
}

impl<RESP, ITEM> ChatResponse<RESP, ITEM>
where
    RESP: DeserializeOwned,
    ITEM: DeserializeOwned,
{
    pub fn must_response(self) -> RESP {
        match self {
            ChatResponse::Full(resp) => resp,
            ChatResponse::Stream(_) => panic!("Expected Full variant, found Stream"),
        }
    }

    pub fn must_stream(self) -> JsonStream<ITEM> {
        match self {
            ChatResponse::Stream(stream) => stream,
            ChatResponse::Full(_) => panic!("Expected Stream variant, found Full"),
        }
    }
}
