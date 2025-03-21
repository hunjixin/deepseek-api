use anyhow::Result;
use clap::Parser;
use deepseek_api::Client;
use deepseek_api::{request::MessageRequest, response::ModelType};
use std::vec;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    pub api_key: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let client = Client::new(&args.api_key);
    let mut history = vec![];

    let mut completions = client.chat();
    let builder = completions
        .chat_builder(vec![])
        .use_model(ModelType::DeepSeekChat)
        .append_user_message("hello");
    let resp = completions.create(builder)?.must_response();

    let mut resp_words = vec![];
    for msg in resp.choices.iter() {
        let resp_msg = MessageRequest::from_message(msg.message.as_ref().expect("message"))?;
        history.push(resp_msg);
        resp_words.push(msg.message.as_ref().expect("message").content.clone());
    }

    for msg in resp_words.iter() {
        msg.split("\n").for_each(|x| println!("{}", x));
    }

    Ok(())
}
