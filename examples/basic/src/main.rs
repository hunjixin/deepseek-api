use anyhow::Result;
use clap::Parser;
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

    let mut completions = client.completions();

    let resp = completions.talk("你好啊").await?;
    println!("resp {:?}", resp);
    Ok(())
}
