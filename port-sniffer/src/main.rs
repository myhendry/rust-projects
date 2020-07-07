use std::env;
use std::net::{IpAddr, TcpStream};

#[derive(Debug)]
struct Config {
    x: String,
    y: i32,
}

impl Config {
    fn new(v: &[String]) -> Result<Config, &'static str> {
        if v.len() > 3 {
            return Err("Too many arguments");
        } else if v.len() < 1 {
            return Err("Too few Arguments");
        }
        Ok(Config {
            x: v[1].clone(),
            y: 32,
        })
    }
}
#[derive(Debug)]
struct Arguments {
    flag: String,
    ipaddress: IpAddr,
    threads: u16,
}

impl Arguments {}

fn main() {
    let v: Vec<String> = env::args().collect();
    let config = Config::new(&v);
    println!("{:?}", config);
}
