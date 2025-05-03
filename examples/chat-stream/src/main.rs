use anyhow::Result;
use clap::Parser;
use deepseek_api::ClientBuilder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    pub api_key: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let client = ClientBuilder::new(args.api_key.clone()).build()?;

    let balances = client.balance()?;
    println!("balances {:?}", balances);

    let models = client.models()?;
    println!("models {:?}", models);

    let mut completions = client.chat();
    let builder = completions
        .fim_builder("def fib(a):", "    return fib(a-1) + fib(a-2)")
        .max_tokens(128)?
        .stream(true);
    let stream = completions.create(builder)?.must_stream();
    for item in stream {
        println!("resp: {:?}", item);
    }

    Ok(())
}
