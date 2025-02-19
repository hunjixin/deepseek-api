use schemars::schema::SchemaObject;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ModelType {
    #[default]
    #[serde(rename = "deepseek-chat")]
    DeepSeekChat,
    #[serde(rename = "deepseek-reasoner")]
    DeepSeekReasoner,
}

impl ModelType {
    pub fn get_pricing_info(&self) -> (f32, f32, f32) {
        match self {
            ModelType::DeepSeekChat => (0.5, 2.0, 8.0),
            ModelType::DeepSeekReasoner => (1.0, 4.0, 16.0),
        }
    }

    pub fn get_limit_info(&self) -> (u32, Option<u32>, u32) {
        match self {
            ModelType::DeepSeekChat => (64, None, 8),
            ModelType::DeepSeekReasoner => (64, Some(32), 8),
        }
    }
}

impl fmt::Display for ModelType {
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub owned_by: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModelResp {
    pub object: String,
    pub data: Vec<Model>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BalanceInfo {
    pub currency: String,
    pub total_balance: String,
    pub granted_balance: String,
    pub topped_up_balance: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BalanceResp {
    pub is_available: bool,
    pub balance_infos: Vec<BalanceInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub parameters: SchemaObject,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: Function,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    pub content: String,
    pub reasoning_content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "insufficient_system_resource")]
    InsufficientSystemResource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogProbWrap {
    pub content: Vec<LogProb>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogProb {
    pub token: String,
    pub logprob: f32,
    pub bytes: Option<Vec<u8>>,
    pub top_logprobs: Vec<TopLogProb>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopLogProb {
    pub token: String,
    pub logprob: f32,
    pub bytes: Option<Vec<u8>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Choice {
    pub finish_reason: FinishReason,
    pub index: usize,
    pub message: Message,
    pub logprobs: Option<LogProbWrap>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub completion_tokens: u64,
    pub prompt_tokens: u64,
    pub prompt_cache_hit_tokens: u64,
    pub prompt_cache_miss_tokens: u64,
    pub total_tokens: u64,
    pub completion_tokens_details: CompletionTokensDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionTokensDetails {
    pub reasoning_tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletion {
    pub id: String,
    pub choices: Vec<Choice>,
    pub created: u32,
    pub model: String,
    pub system_fingerprint: String,
    pub object: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Delta {
    pub content: String,
    pub reasoning_content: String,
    pub role: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChoiceStream {
    pub delta: Delta,
    pub finish_reason: FinishReason,
    pub index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionStream {
    pub id: String,
    pub choices: Vec<ChoiceStream>,
    pub created: u32,
    pub model: String,
    pub system_fingerprint: String,
    pub object: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_get_pricing_info() {
        let chat_model = ModelType::DeepSeekChat;
        let reasoner_model = ModelType::DeepSeekReasoner;

        // Test DeepSeekChat pricing
        let (hit, miss, output) = chat_model.get_pricing_info();
        assert_eq!(hit, 0.5);
        assert_eq!(miss, 2.0);
        assert_eq!(output, 8.0);

        // Test DeepSeekReasoner pricing
        let (hit, miss, output) = reasoner_model.get_pricing_info();
        assert_eq!(hit, 1.0);
        assert_eq!(miss, 4.0);
        assert_eq!(output, 16.0);
    }

    #[test]
    fn test_get_limit_info() {
        let chat_model = ModelType::DeepSeekChat;
        let reasoner_model = ModelType::DeepSeekReasoner;

        // Test DeepSeekChat limits
        let (context_len, thought_chain_len, output_len) = chat_model.get_limit_info();
        assert_eq!(context_len, 64);
        assert_eq!(thought_chain_len, None);
        assert_eq!(output_len, 8);

        // Test DeepSeekReasoner limits
        let (context_len, thought_chain_len, output_len) = reasoner_model.get_limit_info();
        assert_eq!(context_len, 64);
        assert_eq!(thought_chain_len, Some(32));
        assert_eq!(output_len, 8);
    }

    #[test]
    fn test_serialize_deserialize() {
        let chat_model = ModelType::DeepSeekReasoner;

        // Serialize
        let serialized = serde_json::to_string(&chat_model).unwrap();
        assert_eq!(serialized, "\"deepseek-reasoner\"");

        // Deserialize
        let deserialized: ModelType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, chat_model);
    }
}
