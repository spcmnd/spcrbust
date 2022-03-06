use reqwest;
use std::fs::File;
use std::io::{self, BufRead};
use std::net::IpAddr;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;

  Ok(io::BufReader::new(file).lines())
}

pub async fn test_word(target: IpAddr, word: &str) -> Result<(), reqwest::Error> {
  let res = reqwest::get(format!("http://{}/{}", target, word)).await?;

  println!("Word: {} - Status: {}", word, res.status().as_u16());

  Ok(())
}
