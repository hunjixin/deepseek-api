use anyhow::Result;
use clap::Parser;
use seep_seek_api::Client;
use seep_seek_api::request::MaxToken;
use seep_seek_api::response::ModelType;

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
        .max_tokens(MaxToken::new(128)?);
    let resp = completions.create(builder).await?;
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
