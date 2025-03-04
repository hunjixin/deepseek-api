use anyhow::Result;
use clap::Parser;
use seep_seek_api::Client;
use seep_seek_api::request::MaxToken;
use seep_seek_api::response::ModelType;
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

    let mut completions = client.completions().set_model(ModelType::DeepSeekChat);
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
