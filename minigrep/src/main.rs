use std::env;
use std::process;

// use std::fs;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // let filename = &args[2];
    // let contents = fs::read_to_string(filename);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };

    // let args1: Vec<String> = env::args().collect();
    // let zz = args1[0].clone(); //* error move occured
    // println!("zz {}", zz);
    // let v = &args[1];
    // println!("v {}", v); //* this will return &String
}

/*
use std::io;, io::Error
Box<dyn<Error>>
*/
