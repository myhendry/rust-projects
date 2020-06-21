use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
  pub filename: String,
  pub query: String,
}

impl Config {
  pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("Too few arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { filename, query })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  println!("run contents {}", contents);
  Ok(())
}
