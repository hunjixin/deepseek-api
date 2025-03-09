use schemars::schema::SchemaObject;
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
    /// Retrieves the pricing information for the model.
    ///
    /// Returns a tuple containing:
    /// - `hit_price`: Price for cache hit.
    /// - `miss_price`: Price for cache miss.
    /// - `output_price`: Price for output.
    pub fn get_pricing_info(&self) -> (f32, f32, f32) {
        match self {
            ModelType::DeepSeekChat => (0.5, 2.0, 8.0),
            ModelType::DeepSeekReasoner => (1.0, 4.0, 16.0),
        }
    }

    /// Retrieves the limit information for the model.
    ///
    /// Returns a tuple containing:
    /// - `context_len`: Maximum context length.
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
        let (hit_price, miss_price, output_price) = self.get_pricing_info();

        match self {
            ModelType::DeepSeekChat => {
                write!(
                    f,
                    "DeepSeekChat: Context Length = {}K, Max Output Length = {}K, \nInput Price (Cache Hit) = {}元, Input Price (Cache Miss) = {}元, Output Price = {}元",
                    context_len, output_len, hit_price, miss_price, output_price
                )
            }
            ModelType::DeepSeekReasoner => {
                write!(
                    f,
                    "DeepSeekReasoner: Context Length = {}K, Max Thought Chain Length = {:?}K, Max Output Length = {}K, \nInput Price (Cache Hit) = {}元, Input Price (Cache Miss) = {}元, Output Price = {}元",
                    context_len, thought_chain_len, output_len, hit_price, miss_price, output_price
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
    /// Parameters of the function.
    pub parameters: SchemaObject,
}

/// Represents a tool call with its associated function.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ToolCall {
    /// Unique identifier for the tool call.
    pub id: String,
    /// Type of the tool call.
    pub r#type: String,
    /// Function associated with the tool call.
    pub function: Function,
}

/// Represents a message with its content and optional reasoning content and tool calls.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    /// Content of the message.
    pub content: String,
    /// Optional reasoning content.
    pub reasoning_content: Option<String>,
    /// Optional list of tool calls.
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Role of the message sender.
    pub role: String,
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
    pub message: Option<Message>,
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
    pub completion_tokens_details: CompletionTokensDetails,
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
}

/// Represents a delta change in a choice stream.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Delta {
    /// Content of the delta change.
    pub content: String,
    /// Reasoning content of the delta change.
    pub reasoning_content: String,
    /// Role of the delta change sender.
    pub role: String,
}

/// Represents a choice stream with its associated delta change.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChoiceStream {
    /// Delta change in the choice stream.
    pub delta: Delta,
    /// Reason for finishing the choice stream.
    pub finish_reason: FinishReason,
    /// Index of the choice stream.
    pub index: usize,
}

/// Represents a chat completion stream with its associated metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionStream {
    /// Unique identifier for the chat completion stream.
    pub id: Option<String>,
    /// List of choice streams made during the chat completion stream.
    pub choices: Vec<ChoiceStream>,
    /// Timestamp of when the chat completion stream was created.
    pub created: u32,
    /// Model used for the chat completion stream.
    pub model: String,
    /// System fingerprint associated with the chat completion stream.
    pub system_fingerprint: String,
    /// Type of the object.
    pub object: String,
}

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

    /// 返回 `Stream` 变体中的 `JsonStream<ITEM>` 值，如果当前是 `Full` 变体则 panic
    pub fn must_stream(self) -> JsonStream<ITEM> {
        match self {
            ChatResponse::Stream(stream) => stream,
            ChatResponse::Full(_) => panic!("Expected Stream variant, found Full"),
        }
    }
}
