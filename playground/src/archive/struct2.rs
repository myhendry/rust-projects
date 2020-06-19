struct B {
  x: String,
}

impl B {
  fn new(a: &str) -> Self {
    B { x: String::from(a) }
  }
}

impl Base for B {
  fn read(&self, s: &str) -> String {
    format!("{} {}", self.x, s)
  }
}

#[derive(Debug)]
enum Gender {
  _Male(i32),
  Female(i32),
}

trait Base {
  fn read(&self, s: &str) -> String;
}

#[derive(Debug)]
struct A<'a, F>
where
  F: Fn(i32) -> i32,
{
  x: &'a str,  //* Slice Literals with Lifetimes
  y: &'a str,  //* Slice Literals with Lifetimes
  c: F,        //* Closures
  g: Gender,   //* Enums
  v: Vec<i32>, //* Vectors
}

impl<'a, F> A<'a, F>
where
  F: Fn(i32) -> i32,
{
  fn new(a: &'a str, b: &'a str, f: F, g: Gender, v: Vec<i32>) -> Self {
    A {
      x: a,
      y: b,
      c: f,
      g,
      v,
    }
  }

  fn name(&self) -> String {
    format!("{} {}", self.x, self.y)
  }

  fn cycle(&self) -> Vec<i32> {
    self.v.iter().map(|x| *x + 1).collect()
  }
}

impl<'a, F> Base for A<'a, F>
where
  F: Fn(i32) -> i32,
{
  fn read(&self, s: &str) -> String {
    format!("{} {}", self.x, s)
  }
}

fn gen_read<T>(s: &T, ss: &str) -> String
where
  T: Base,
{
  s.read(ss)
}

fn run() {
  let cc = |x| x * 10;
  let vv = vec![10, 20, 30];

  let a2 = A::new("orange", "papaya", cc, Gender::Female(32), vv);
  println!("{}", a2.x);
  println!("{}", a2.y);
  let ccr1 = (a2.c)(12);
  println!("{}", ccr1);
  println!("{:?}", a2.g);
  if let Gender::Female(x) = a2.g {
    println!("{}", x);
  };
  match a2.g {
    Gender::Female(x) => println!("{}", x),
    _ => println!("{}", "Others"),
  }
  println!("{}", a2.name());
  for x in a2.v.iter() {
    println!("{}", x);
  }
  let vv2: Vec<_> = a2.v.iter().map(|x| x + 1).collect();
  println!("{:?}", vv2);
  println!("{}", a2.read("heeyyyy"));

  let b = B::new("Firminho");
  println!("{}", b.read("Dribbles"));

  let r1 = gen_read(&b, "nice");
  println!("{}", r1);

  let r2 = gen_read(&a2, "cool");
  println!("{}", r2);

  let vv3 = a2.cycle();
  println!("{:?}", vv3);
}
