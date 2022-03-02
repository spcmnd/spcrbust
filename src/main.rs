use clap::Parser;
use std::net::IpAddr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub target: IpAddr,

    #[clap(short, long)]
    pub word_list: String,

    #[clap(short, long)]
    pub output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let target = args.target;
    let word_list = args.target;
    let output = match args.output {
        None => String::new(),
        Some(o) => o,
    };
    println!("Hello, world!");
}
