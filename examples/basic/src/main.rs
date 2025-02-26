use anyhow::Result;
use clap::Parser;
use seep_seek_api::response::ModelType;
use seep_seek_api::Client;

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
    let builder = completions.chat_builder().append_user_message("你好啊");
    let resp = completions.create(builder).await?;

    println!("resp {:?}", resp);
    Ok(())
}
