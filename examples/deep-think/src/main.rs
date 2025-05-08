use anyhow::Result;
use clap::Parser;
use deepseek_api::{
    ClientBuilder, CompletionsRequestBuilder, RequestBuilder,
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

    let completions = client.chat();
    let mut stream = CompletionsRequestBuilder::new(vec![MessageRequest::User(
        UserMessageRequest::new("how to get to beijing"),
    )])
    .use_model(ModelType::DeepSeekReasoner)
    .stream(true)
    .do_request(&completions)
    .await?
    .must_stream();
    while let Some(item) = stream.next().await {
        println!("resp: {:?}", item);
    }
    Ok(())
}
