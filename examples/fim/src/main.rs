use anyhow::Result;
use clap::Parser;
use deepseek_api::{ClientBuilder, FMICompletionsRequestBuilder, RequestBuilder};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    pub api_key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let client = ClientBuilder::new(args.api_key.clone()).build()?;

    let balances = client.balance().await?;
    println!("balances {:?}", balances);

    let models = client.models().await?;
    println!("models {:?}", models);

    let completions = client.chat();
    let resp = FMICompletionsRequestBuilder::new("def fib(a):", "    return fib(a-1) + fib(a-2)")
        .max_tokens(128)?
        .do_request(&completions)
        .await?
        .must_response();
    println!(
        "resp {:?}",
        resp.choices
            .first()
            .as_ref()
            .unwrap()
            .text
            .as_ref()
            .unwrap()
    );
    Ok(())
}
