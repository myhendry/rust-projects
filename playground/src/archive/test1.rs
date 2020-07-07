mod example;

use example::err1;
//use example::sub1;

#[derive(Debug)]
struct Point<T> {
  x: String,
  y: u32,
  z: T,
  u: Gender,
}

#[derive(Debug)]
enum Gender {
  Male(u32),
  Female(u32),
}

impl Gender {
  fn find_age(&self) -> u32 {
    match self {
      Gender::Male(x) => *x,
      Gender::Female(y) => *y,
    }
  }
}

impl Point<f32> {
  fn new(x: &str, y: u32, z: f32, u: Gender) -> Self {
    Self {
      x: String::from(x),
      y,
      z,
      u,
    }
  }

  fn print(&self) {
    println!("{} {} {}", self.x, self.y, self.z);
  }
}

impl<T> Point<T>
where
  T: std::ops::Add<Output = T>,
{
  fn rec(x: &str, y: u32, z: T, u: Gender) -> Self {
    Self {
      x: String::from(x),
      y,
      z,
      u,
    }
  }

  fn read(&self) {
    println!("add {}", self.y);
  }

  fn analyze(&self, u: u32) -> Option<u32> {
    if u > 10 {
      return Some(20);
    }
    None
  }
}

fn run() {
  err1::run();

  let s = String::from("hello world");
  let index_first_word = first_word(&s);
  println!("{}", index_first_word);

  let p1 = Point::new("apple", 20, 30.0, Gender::Female(25));
  println!("{:?}", p1);
  p1.print();
  println!("r1 {}", p1.u.find_age());

  let p2 = Point::rec("hey", 22, 33, Gender::Male(28));
  p2.read();
  let r2 = p2.u.find_age();
  println!("r2 {}", r2);
  let r3 = p2.analyze(88);
  if let Some(x) = r3 {
    println!("r3 {}", x);
  }
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }
  &s[..]
}
