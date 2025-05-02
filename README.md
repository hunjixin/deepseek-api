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

## Installation

Add the DeepSeek API SDK to your project by including it in your `Cargo.toml`:

todo


## Usage
You can use the DeepSeek API SDK with either asynchronous or synchronous code, depending on your application's requirements. Below are examples demonstrating both approaches:
### Asynchronous Example
```rust
use anyhow::Result;
use clap::Parser;
use deepseek_api::{Client, request::MessageRequest, response::ModelType};
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
    let client = Client::new(&args.api_key);
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
                let mut completions = client.chat();
                let builder = completions.chat_builder(vec![]).append_user_message(word);
                let resp = completions.create(builder).await?.must_response();

                let mut resp_words = vec![];
                for msg in resp.choices.iter() {
                    let resp_msg =
                        MessageRequest::from_message(msg.message.as_ref().expect("message"))?;
                    history.push(resp_msg);
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

### Synchronous Example
use `default-features = false, features = ["is_sync"]` feature to use sync api
```rs
use anyhow::Result;
use clap::Parser;
use deepseek_api::Client;
use deepseek_api::{request::MessageRequest, response::ModelType};
use std::vec;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    pub api_key: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let client = Client::new(&args.api_key);
    let mut history = vec![];

    let mut completions = client.chat();
    let builder = completions
        .chat_builder(vec![])
        .use_model(ModelType::DeepSeekChat)
        .append_user_message("hello");
    let resp = completions.create(builder)?.must_response();

    let mut resp_words = vec![];
    for msg in resp.choices.iter() {
        let resp_msg = MessageRequest::from_message(msg.message.as_ref().expect("message"))?;
        history.push(resp_msg);
        resp_words.push(msg.message.as_ref().expect("message").content.clone());
    }

    for msg in resp_words.iter() {
        msg.split("\n").for_each(|x| println!("{}", x));
    }

    Ok(())
}

```
