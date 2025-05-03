use anyhow::Result;
use clap::Parser;
use deepseek_api::response::ModelType;
use deepseek_api::ClientBuilder;
use std::io::{stdin, stdout, Write};
use std::vec;

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
    loop {
        let mut buffer = String::new();

        print!(">");
        let _ = stdout().flush();
        // `read_line` returns `Result` of bytes read
        stdin().read_line(&mut buffer)?;

        let mut history = vec![];

        match buffer.trim_end() {
            "" => {
                println!("Input you question.");
            }
            "exit" => {
                break;
            }
            "balance" => {
                let balances = client.balance().await?;
                println!("balances {:?}", balances);
            }
            "models" => {
                let models = client.models().await?;
                println!("models {:?}", models);
            }
            word => {
                let mut completions = client.chat();
                let builder = completions
                    .chat_builder(vec![])
                    .use_model(ModelType::DeepSeekChat)
                    .append_user_message(word);
                let resp = completions.create(builder).await?.must_response();

                let mut resp_words = vec![];
                for msg in resp.choices.iter() {
                    history.push(msg.message.as_ref().expect("message exit").clone());
                    resp_words.push(msg.message.as_ref().expect("message").content.clone());
                }

                for msg in resp_words.iter() {
                    msg.split("\n").for_each(|x| println!("{}", x));
                }
            }
        };
    }
    Ok(())
}
