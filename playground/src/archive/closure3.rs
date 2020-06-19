struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn new(arg1: T, arg2: U) -> Self {
    Point { x: arg1, y: arg2 }
  }
}

struct Rect {
  w: u32,
  h: u32,
  name: String,
}

impl Rect {
  fn new(w: u32, h: u32, n: &str) -> Self {
    Self {
      w,
      h,
      name: String::from(n),
    }
  }

  fn area(&self) -> u32 {
    self.w * self.h
  }
}

impl Base for Rect {
  fn read(&self, arg1: &str) -> String {
    format!("{} {}", self.name, arg1)
  }
}

struct Circle {
  w: u32,
  name: String,
}

impl Circle {
  fn new(w: u32, n: &str) -> Self {
    Circle {
      w,
      name: String::from(n),
    }
  }

  fn area(&self) -> u32 {
    self.w * self.w
  }
}

impl Base for Circle {
  fn read(&self, arg1: &str) -> String {
    format!("{} {}", &self.name, arg1)
  }
}

trait Base {
  fn read(&self, arg1: &str) -> String;
}

fn common<T>(arg1: &T, arg2: &str) -> String
where
  T: Base,
{
  arg1.read(arg2)
}

struct Calculator<F>
where
  F: Fn(u32) -> u32,
{
  c: F,
}

impl<F> Calculator<F>
where
  F: Fn(u32) -> u32,
{
  fn new(f: F) -> Self {
    Calculator { c: f }
  }
}

struct Math<F>
where
  F: Fn(u32) -> u32,
{
  name: String,
  x: F,
}

impl<F> Math<F>
where
  F: Fn(u32) -> u32,
{
  fn new(f: F, n: &str) -> Self {
    Math {
      x: f,
      name: String::from(n),
    }
  }

  fn intro(&self, arg1: &str) -> String {
    format!("{} {}", arg1, self.name)
  }
}

fn ccf1<F>(f: F, u: u32) -> u32
where
  F: Fn(u32) -> u32,
{
  f(u)
}

fn run() {
  let cc2 = |x| x * 22;
  let ccr1 = ccf1(cc2, 30);
  println!("ccr1 {}", ccr1);

  let cl1 = |x| x + 1000;
  let cc = Calculator::new(cl1);
  let cl2 = cc.c;
  println!("cl2 {}", cl2(22));
  let mm = Math::new(cl1, "Welcome");
  let mm1 = (mm.x)(32);
  println!("mm1 {}", mm1);
  println!("{}", mm.intro("yesss"));

  let p1 = Point::new(3, 2.5);
  println!("{} {}", p1.x, p1.y);
  println!("{} {}", p1.x, p1.y);

  let r1 = Rect::new(20, 10, "John");
  println!("{}", r1.area());
  println!("{}", r1.w);
  println!("{}", r1.read("Alexx"));

  let r2 = Rect::new(50, 20, "Alvin");
  println!("{}", r2.area());

  let c1 = Circle::new(20, "Irene");
  println!("{}", c1.area());

  let s1 = common(&c1, "Hi");
  println!("{}", s1);

  let s2 = common(&r2, "Hey");
  println!("{}", s2);
}
s
