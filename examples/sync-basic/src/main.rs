use anyhow::Result;
use clap::Parser;
use deepseek_api::{request::MessageRequest, response::ModelType};
use deepseek_api::{CompletionsRequestBuilder, DeepSeekClientBuilder, RequestBuilder};
use std::vec;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    pub api_key: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let client = DeepSeekClientBuilder::new(args.api_key.clone())
        .timeout(300)
        .build()?;

    let mut history = vec![];
    let resp = CompletionsRequestBuilder::new(&[MessageRequest::user("hello world")])
        .use_model(ModelType::DeepSeekReasoner)
        .do_request(&client)?
        .must_response();

    let mut resp_words = vec![];
    for msg in resp.choices.iter() {
        history.push(MessageRequest::Assistant(
            msg.message.as_ref().expect("message exit").clone(),
        ));
        resp_words.push(msg.message.as_ref().expect("message").content.clone());
    }
    for msg in resp_words.iter() {
        msg.split("\n").for_each(|x| println!("{}", x));
    }

    Ok(())
}
