#![allow(unused_variables)]
#![allow(dead_code)]
use minigrep::Config;
use std::env;
use std::fs;
use std::io;
use std::ops::Mul;
use std::process;

mod example;

// use crate::example::sub1;
use example::sub1;

trait Shape<T> {
  fn area(&self) -> T;
}

struct Rectangle<T: Mul> {
  x: T,
  y: T,
}

impl<T: Copy> Shape<T> for Rectangle<T>
where
  T: Mul<Output = T>,
{
  fn area(&self) -> T {
    self.x * self.y
  }
}

//*! using struct with Iterator trait
#[derive(Debug)]
struct Fibonacchi {
  curr: u32,
  next: u32,
}

impl Iterator for Fibonacchi {
  type Item = u32; //* associated type
  fn next(&mut self) -> Option<Self::Item> {
    let new_next = self.curr + self.next;
    self.curr = self.next;
    self.next = new_next;

    Some(self.curr)
  }
}

//*! Trial & Error
fn function_with_error(arg: bool) -> Result<u64, &'static str> {
  if arg {
    return Err("Error in function with error");
  }

  Ok(255)
}

fn read_file() -> Result<String, io::Error> {
  // let mut s = String::new();
  let ff = fs::File::open("text.txt");
  // let pp = fs::File::open("text.txt")?.read_to_string(&mut s)?;
  let ff2 = function_with_error(true);
  println!("ff2 {:?}", ff2);
  let ff3 = function_with_error(false).unwrap();
  println!("ff3 {}", ff3);
  let ff4 = function_with_error(true).expect("error in function with error");
  println!("ff4 {}", ff4);
  Ok("abc".to_string())
}

fn run() {
  sub1::run();

  //*! Trial & Error
  // let zu = read_file();
  // println!("{:?}", zu);

  // let ww;
  // match function_with_error(false) {
  //   Ok(v) => ww = v,
  //   Err(e) => panic!(e),
  // }
  // println!("ww {}", ww);

  // let data: Vec<String> = env::args().collect();
  // let yy1 = data.get(1).unwrap();
  // let yy2 = data.get(2).expect("Insufficient arguments");
  // println!("YY {} {}", yy1, yy2);

  // println!("{:?}", data);
  // for d in data.iter() {
  //   println!("{}", d);
  // }
  // let file: &String = &data[1];
  // println!("file {}", file);

  //*! Minigrep
  let data: Vec<String> = env::args().collect();
  //* Result enum Result<Config, &'static str> can use unwrap_or_else
  let config = Config::new(&data).unwrap_or_else(|err| {
    println!("Err at Config::new {}", err);
    process::exit(1);
  });
  println!("config {:?}", config);

  //* io::Result<String> use expect
  //let contents = fs::read_to_string(&config.filename).expect("Err at read_to_string");
  //println!("contents {}", contents);

  //* if no error, will run successfully in lib else error will be propagated to be handled in main.rs
  if let Err(e) = minigrep::run(config) {
    println!("Err at run(config) {}", e);
    process::exit(1);
  }

  //*!  use unwrap with OK enum and expect with Result enum? Option enum can use both expect and unwrap?
  let pp = Some(10);
  println!("pp {}", pp.unwrap());

  let r = Rectangle { x: 10, y: 20 };
  let r2 = Rectangle { x: 10.10, y: 20.20 };
  println!("{} {}", r.area(), r2.area());

  //*! for in with iter() vector
  let v1 = vec![10, 20, 30];
  for v in v1.iter() {
    println!("v1 {}", v);
  }
  println!("{:?}", v1);

  //*! for in with into_iter() vector
  let v2 = vec![20, 30, 40];
  for v in v2.into_iter() {
    println!("v2 {}", v);
  }
  // println!("{:?}", v2); //* Error - Borrow of moved value

  //*! for in with iter_mut() vector
  let mut v3 = vec![40, 50, 60];
  for v in v3.iter_mut() {
    *v *= 10;
    println!("v3 {}", v);
  }
  println!("{:?}", v3);

  //*! for in with vector
  let v4 = vec![50, 60, 70];
  for v in v4 {
    println!("v4 {}", v);
  }
  // println!("{:?}", v4); //* Error - Borrow of moved value

  //*! for in with ref vector
  let v5 = vec![60, 70, 80];
  for v in &v5 {
    println!("v5 {}", v);
  }
  println!("{:?}", v5);

  //*! using Struct with Iterator trait
  let mut fib = Fibonacchi { curr: 1, next: 1 };
  for _ in 0..5 {
    println!("fib.curr {:?}", fib.curr);
    fib.next();
  }
  println!("fib {:?}", fib);

  //*! Borrowing and Ownership
  let mut s1 = String::from("Hi");
  let s2 = &s1;
  println!("{}", s2);
  println!("{}", s1);
  let s3 = &mut s1;
  s3.push_str("Ho");

  // println!("{}", s2); //* cannot borrow `s1` as mutable because it is also borrowed as immutable
  println!("s3 {}", s3);
  println!("s1 {}", s1);
  //println!("{}", s2); //* cannot borrow `s1` as mutable because it is also borrowed as immutable

  let s4 = &mut s1;
  s4.push_str("Wow");
  println!("{}", s4);
  println!("{}", s1);
  // println!("{}", s3); //* cannot borrow `s1` as mutable more than once at a time
  // println!("{}", s2); //* cannot borrow `s1` as mutable because it is also borrowed as immutable
}
