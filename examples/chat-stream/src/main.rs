use anyhow::Result;
use clap::Parser;
use deepseek_api::Client;
use tokio_stream::StreamExt;

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

    let balances = client.balance().await?;
    println!("balances {:?}", balances);

    let models = client.models().await?;
    println!("models {:?}", models);

    let mut completions = client.chat();
    let builder = completions
        .fim_builder("def fib(a):", "    return fib(a-1) + fib(a-2)")
        .max_tokens(128)?
        .stream(true);
    let mut stream = completions.create(builder).await?.must_stream();
    while let Some(item) = stream.next().await {
        println!("resp: {:?}", item);
    }
    Ok(())
}
