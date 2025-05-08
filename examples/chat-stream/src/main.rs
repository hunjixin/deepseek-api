use anyhow::Result;
use clap::Parser;
use deepseek_api::{DeepSeekClientBuilder, FMICompletionsRequestBuilder, RequestBuilder};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    pub api_key: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let client = DeepSeekClientBuilder::new(args.api_key.clone()).build()?;

    let balances = client.balance()?;
    println!("balances {:?}", balances);

    let models = client.models()?;
    println!("models {:?}", models);

    let stream = FMICompletionsRequestBuilder::new("def fib(a):", "    return fib(a-1) + fib(a-2)")
        .max_tokens(128)?
        .stream(true)
        .do_request(&client)?
        .must_stream();
    for item in stream {
        println!("resp: {:?}", item);
    }

    Ok(())
}
