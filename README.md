# DeepSeek API SDK

DeepSeek API SDK is a Rust client library for interacting with the DeepSeek service. It provides a high-performance, safe, and easy-to-use interface for making API calls to DeepSeek, enabling seamless integration into your Rust applications.

---

## Overview & Features

The DeepSeek API SDK simplifies communication with the DeepSeek backend by offering an intuitive API design that allows you to easily send requests and process responses. Built with Rust, the SDK leverages the language’s performance and memory safety guarantees to help you build robust and scalable applications. Key benefits include:

- ✅ **High Performance:** Utilizes Rust’s concurrency and low-level optimizations for fast API interactions.
- ✅ **Memory Safety:** Leverages Rust’s strong type system and ownership model to minimize runtime errors.
- ✅ **Intuitive Interface:** Simplifies API usage with clear methods and comprehensive error handling.
- ✅ **Async & Blocking Support:** Offers both asynchronous (e.g., using Tokio) and synchronous (blocking) interfaces to suit different application needs.

---

## Use Cli
This project comes with a command-line tool `ds-cli`, which provides a similar user experience to the web version. You can use it to interact with DeepSeek.

```bash
cargo install ds-cli
ds-cli --api-key <your api key>
```
![Image](https://github.com/user-attachments/assets/28b58387-f56c-4583-bc94-7afb54392edb)


## Use in you code

Add the DeepSeek API SDK to your project by including it in your `Cargo.toml`:

```bash
cargo add deepseek-api
```

## Usage

The DeepSeek API SDK supports both asynchronous and synchronous usage patterns in Rust, giving you flexibility based on your runtime and application needs. You can also leverage function calling features for advanced interactions like calling custom-defined tools.

### Asynchronous Example  (Recommended for Most Use Cases)
```rust
use anyhow::Result;
use clap::Parser;
use deepseek_api::response::ModelType;
use deepseek_api::{DeepSeekClientBuilder, CompletionsRequestBuilder, RequestBuilder};
use std::io::{stdin, stdout, Write};
use std::vec;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    pub api_key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = DeepSeekClientBuilder::new(args.api_key.clone()).build()?;
    loop {
        let mut buffer = String::new();

        print!(">");
        let _ = stdout().flush();
        // `read_line` returns `Result` of bytes read
        stdin().read_line(&mut buffer)?;

        let mut history = vec![];

        match buffer.trim_end() {
            "" => {
                println!("Input you question.");
            }
            "exit" => {
                break;
            }
            "balance" => {
                let balances = client.balance().await?;
                println!("balances {:?}", balances);
            }
            "models" => {
                let models = client.models().await?;
                println!("models {:?}", models);
            }
            word => {
                let completions = client.chat();
                let resp = CompletionsRequestBuilder::new(vec![])
                    .use_model(ModelType::DeepSeekChat)
                    .append_user_message(word)
                    .do_request(&completions)
                    .await?
                    .must_response();

                let mut resp_words = vec![];
                for msg in resp.choices.iter() {
                    history.push(msg.message.as_ref().expect("message exit").clone());
                    resp_words.push(msg.message.as_ref().expect("message").content.clone());
                }

                for msg in resp_words.iter() {
                    msg.split("\n").for_each(|x| println!("{}", x));
                }
            }
        };
    }
    Ok(())
}
```

### Synchronous Example  (Requires Feature Flag)
To use the synchronous version, disable default features and enable is_sync:
```examples
deepseek-api = { version = "xx", default-features = false, features = ["is_sync"] }
```

```rs
use anyhow::Result;
use clap::Parser;
use deepseek_api::{request::MessageRequest, response::ModelType};
use deepseek_api::{DeepSeekClientBuilder, CompletionsRequestBuilder, RequestBuilder};
use std::vec;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    pub api_key: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let client = DeepSeekClientBuilder::new(args.api_key.clone())
        .timeout(300)
        .build()?;
    let mut history = vec![];

    let completions = client.chat();
    let resp = CompletionsRequestBuilder::new(vec![])
        .use_model(ModelType::DeepSeekReasoner)
        .append_user_message("hello world")
        .do_request(&completions)?
        .must_response();

    let mut resp_words = vec![];
    for msg in resp.choices.iter() {
        history.push(MessageRequest::Assistant(
            msg.message.as_ref().expect("message exit").clone(),
        ));
        resp_words.push(msg.message.as_ref().expect("message").content.clone());
    }
    for msg in resp_words.iter() {
        msg.split("\n").for_each(|x| println!("{}", x));
    }

    Ok(())
}
```

### Function Calling

Use the function calling interface to define and invoke tools via the API.

```rust
use anyhow::Result;
use clap::Parser;
use deepseek_api::request::MessageRequest;
use deepseek_api::request::{
    Function, ToolMessageRequest, ToolObject, ToolType, UserMessageRequest,
};
use deepseek_api::response::FinishReason;
use deepseek_api::{DeepSeekClientBuilder, CompletionsRequestBuilder, RequestBuilder};
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
    let completetion = client.chat();
    let resp = CompletionsRequestBuilder::new(messages.clone())
        .tools(vec![tool_object.clone()])
        .do_request(&completetion)
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
        .do_request(&completetion)
        .await?
        .must_response();
    println!(
        "Reply with my function: {:?}",
        resp.choices[0].message.as_ref().unwrap().content
    );
    Ok(())
}
```
