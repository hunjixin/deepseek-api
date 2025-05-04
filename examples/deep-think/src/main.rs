use anyhow::Result;
use clap::Parser;
use deepseek_api::{
    ClientBuilder,
    request::{MessageRequest, UserMessageRequest},
    response::ModelType,
};
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

    let client = ClientBuilder::new(args.api_key.clone()).build()?;

    let mut completions = client.chat();
    let builder = completions
        .chat_builder(vec![MessageRequest::User(UserMessageRequest::new(
            "how to get to beijing",
        ))])
        .use_model(ModelType::DeepSeekReasoner)
        .stream(true);
    let mut stream = completions.create(builder).await?.must_stream();
    while let Some(item) = stream.next().await {
        println!("resp: {:?}", item);
    }
    Ok(())
}
