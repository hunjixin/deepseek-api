use anyhow::Result;
use clap::Parser;
use deepseek_api::request::MessageRequest;
use deepseek_api::request::{
    Function, ToolMessageRequest, ToolObject, ToolType, UserMessageRequest,
};
use deepseek_api::response::FinishReason;
use deepseek_api::{CompletionsRequestBuilder, DeepSeekClientBuilder, RequestBuilder};
use schemars::schema::SchemaObject;
use std::vec;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    pub api_key: String,
}

/// This example demonstrates how to use function calling in the DeepSeek API.
/// It defines a function to get the weather of a location, and then calls that function
/// based on the user's input. The function is defined using JSON Schema, and the API
/// will automatically parse the user's input to match the function's parameters.
/// More detail can refer to https://api-docs.deepseek.com/guides/function_calling
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = DeepSeekClientBuilder::new(args.api_key.clone()).build()?;
    let parameters: SchemaObject = serde_json::from_str(
        r#"{
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "The location to get the weather for"
            },
            "unit": {
                "type": "string",
                "enum": ["celsius", "fahrenheit"],
                "description": "The unit of temperature"
            }
        },
        "required": ["location"]
    }"#,
    )?;

    let tool_object = ToolObject {
        tool_type: ToolType::Function,
        function: Function {
            name: "get_weather".to_string(),
            description: "Get weather of an location, the user shoud supply a location first"
                .to_string(),
            parameters,
        },
    };

    let mut messages = vec![MessageRequest::User(UserMessageRequest::new(
        "How's the weather in Hangzhou?",
    ))];
    let resp = CompletionsRequestBuilder::new(messages.clone())
        .tools(vec![tool_object.clone()])
        .do_request(&client)
        .await?
        .must_response();
    let mut id = String::new();
    if resp.choices[0].finish_reason == FinishReason::ToolCalls {
        if let Some(msg) = &resp.choices[0].message {
            if let Some(tool) = &msg.tool_calls {
                id = tool[0].id.clone();
                println!("Function id: {}", id);
                println!("Function name: {}", tool[0].function.name);
                println!("Function parameters: {:?}", tool[0].function.arguments);
            }
            messages.push(MessageRequest::Assistant(msg.clone()));
        }
    }

    messages.push(MessageRequest::Tool(ToolMessageRequest::new("24℃", &id)));
    let resp = CompletionsRequestBuilder::new(messages.clone())
        .tools(vec![tool_object.clone()])
        .do_request(&client)
        .await?
        .must_response();
    println!(
        "Reply with my function: {:?}",
        resp.choices[0].message.as_ref().unwrap().content
    );
    Ok(())
}
