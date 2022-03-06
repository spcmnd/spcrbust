use clap::Parser;
use reqwest;
use std::net::IpAddr;

mod lib;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub target: IpAddr,

    #[clap(short, long)]
    pub word_list: String,

    #[clap(short('x'))]
    pub extensions: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    let target = args.target;
    let word_list = args.word_list;
    let extensions: Vec<&str> = match args.extensions {
        None => Vec::new(),
        Some(ref x) => x.split(",").collect(),
    };

    if let Ok(lines) = lib::read_lines(word_list) {
        for line in lines {
            if let Ok(l) = line {
                lib::test_word(target, &l).await?;

                if !extensions.is_empty() {
                    for ext in &extensions {
                        lib::test_word(target, &format!("{}.{}", &l, ext)).await?;
                    }
                }
            }
        }
    }

    Ok(())
}
