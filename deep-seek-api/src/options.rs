use schemars::schema::SchemaObject;
use serde::{Deserialize, Serialize};

use crate::{LogProb, ModelType, ToolCall};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrequencyPenalty(pub f32);

impl FrequencyPenalty {
    pub fn new(v: f32) -> Result<Self, String> {
        if v < -2.0 || v > 2.0 {
            return Err("Frequency penalty value must be between -2 and 2.".to_string());
        }
        Ok(FrequencyPenalty(v))
    }
}

impl Default for FrequencyPenalty {
    fn default() -> Self {
        FrequencyPenalty(0.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PresencePenalty(pub f32);

impl PresencePenalty {
    pub fn new(v: f32) -> Result<Self, String> {
        if v < -2.0 || v > 2.0 {
            return Err("Presence penalty value must be between -2 and 2.".to_string());
        }
        Ok(PresencePenalty(v))
    }
}

impl Default for PresencePenalty {
    fn default() -> Self {
        PresencePenalty(0.0)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ResponseType {
    #[serde(rename = "json_object")]
    Json,
    #[serde(rename = "text")]
    Text,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseForamt {
    #[serde(rename = "type")]
    pub resp_type: ResponseType,
}

impl ResponseForamt {
    pub fn new(rt: ResponseType) -> Self {
        ResponseForamt { resp_type: rt }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaxToken(pub u32);

impl MaxToken {
    pub fn new(v: u32) -> Result<Self, String> {
        if v < 1 || v > 8192 {
            return Err("Max token must be between 1 and 8192.".to_string());
        }
        Ok(MaxToken(v))
    }
}

impl Default for MaxToken {
    fn default() -> Self {
        MaxToken(4096)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stop {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamOptions {
    pub include_usage: bool,
}

impl StreamOptions {
    pub fn new(include_usage: bool) -> Self {
        StreamOptions { include_usage }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Temperature(pub u32);

impl Temperature {
    pub fn new(v: u32) -> Result<Self, String> {
        if v > 2 {
            return Err("Temperature must be between 0 and 2.".to_string());
        }
        Ok(Temperature(v))
    }
}

impl Default for Temperature {
    fn default() -> Self {
        Temperature(1)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopP(pub f32);

impl TopP {
    pub fn new(v: f32) -> Result<Self, String> {
        if v < 0.0 || v > 1.0 {
            return Err("TopP value must be between 0and 2.".to_string());
        }
        Ok(TopP(v))
    }
}

impl Default for TopP {
    fn default() -> Self {
        TopP(1.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolType {
    #[serde(rename = "function")]
    Function,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub description: String,
    pub name: String,
    pub parameters: SchemaObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolObject {
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    pub function: Function,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionToolChoice {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionChoice {
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionNamedToolChoice {
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    pub function: FunctionChoice,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolChoice {
    ChatCompletion(ChatCompletionToolChoice),
    ChatCompletionNamed(ChatCompletionNamedToolChoice),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopLogprobs(pub u32);

impl TopLogprobs {
    pub fn new(v: u32) -> Result<Self, String> {
        if v > 20 {
            return Err("Top log probs must be between 0 and 20.".to_string());
        }
        Ok(TopLogprobs(v))
    }
}

impl Default for TopLogprobs {
    fn default() -> Self {
        TopLogprobs(0)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageRequest {
    pub content: String,
    pub role: String,
    pub name: Option<String>,
}
impl MessageRequest {
    pub fn new(msg: &str) -> Self {
        MessageRequest {
            content: msg.to_string(),
            role: "system".to_string(),
            name: None,
        }
    }
    pub fn new_with_name(name: &str, msg: &str) -> Self {
        MessageRequest {
            content: msg.to_string(),
            role: "system".to_string(),
            name: Some(name.to_string()),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use schemars::schema::SchemaObject;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TestStruct {
        pub frequency_penalty: FrequencyPenalty,
        pub presence_penalty: PresencePenalty,
        pub response_format: ResponseForamt,
        pub max_token: MaxToken,
        pub stop: Stop,
        pub stream_options: StreamOptions,
        pub temperature: Temperature,
        pub top_p: TopP,
        pub tool_object: ToolObject,
        pub top_logprobs: TopLogprobs,
    }

    // Test Default values
    #[test]
    fn test_default_values() {
        let default_instance = TestStruct {
            frequency_penalty: FrequencyPenalty::default(),
            presence_penalty: PresencePenalty::default(),
            response_format: ResponseForamt::new(ResponseType::Json),
            max_token: MaxToken::default(),
            stop: Stop::Single("".to_string()), // Default to an empty string for Single variant
            stream_options: StreamOptions::new(true),
            temperature: Temperature::default(),
            top_p: TopP::default(),
            tool_object: ToolObject {
                tool_type: ToolType::Function,
                function: Function {
                    description: "test function".to_string(),
                    name: "test_function".to_string(),
                    parameters: SchemaObject::default(),
                },
            },
            top_logprobs: TopLogprobs::default(),
        };

        // Check default values
        assert_eq!(default_instance.frequency_penalty, FrequencyPenalty(0.0));
        assert_eq!(default_instance.presence_penalty, PresencePenalty(0.0));
        assert_eq!(
            default_instance.response_format.resp_type,
            ResponseType::Json
        );
        assert_eq!(default_instance.max_token, MaxToken(4096));
        assert_eq!(default_instance.stop, Stop::Single("".to_string()));
        assert_eq!(default_instance.stream_options.include_usage, true);
        assert_eq!(default_instance.temperature, Temperature(1));
        assert_eq!(default_instance.top_p, TopP(1.0));
        assert_eq!(default_instance.tool_object.tool_type, ToolType::Function);
        assert_eq!(default_instance.top_logprobs, TopLogprobs(0));
    }

    // Test Serialization and Deserialization
    #[test]
    fn test_serialization_deserialization() {
        let test_instance = TestStruct {
            frequency_penalty: FrequencyPenalty(1.5),
            presence_penalty: PresencePenalty(-1.0),
            response_format: ResponseForamt::new(ResponseType::Text),
            max_token: MaxToken(2048),
            stop: Stop::Multiple(vec!["stop1".to_string(), "stop2".to_string()]),
            stream_options: StreamOptions::new(false),
            temperature: Temperature(2),
            top_p: TopP(0.9),
            tool_object: ToolObject {
                tool_type: ToolType::Function,
                function: Function {
                    description: "sample function".to_string(),
                    name: "sample_func".to_string(),
                    parameters: SchemaObject::default(),
                },
            },
            top_logprobs: TopLogprobs(10),
        };

        // Serialize the struct to JSON
        let serialized = serde_json::to_string(&test_instance).expect("Failed to serialize");

        // Deserialize back to TestStruct
        let deserialized: TestStruct =
            serde_json::from_str(&serialized).expect("Failed to deserialize");

        // Assert that the original instance and deserialized instance are equal
        assert_eq!(test_instance, deserialized);
    }
}
