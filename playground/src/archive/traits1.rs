trait Base {
  fn describe(&self) -> String;
}

#[derive(Debug)]
struct A<T> {
  x: T,
  y: T,
}

impl<T> A<T>
where
  T: std::ops::Mul<Output = T> + Copy + std::fmt::Display,
{
  fn new(x: T, y: T) -> Self {
    A { x, y }
  }

  fn area(&self) -> T {
    self.x * self.y
  }

  fn read(&self) -> String {
    format!("{} {} {}", "A attributes are", self.x, self.y)
  }
}

impl<T> Base for A<T>
where
  T: std::fmt::Display,
{
  fn describe(&self) -> String {
    format!("{} {} {}", "Describe ", self.x, self.y)
  }
}

fn main() {
  let b = calculate(12, 10, 30);
  println!("b {}", b);

  let a = A::new(200, 30);
  println!("{}", a.x);
  println!("{}", a.area());
  println!("{}", a.read());
  println!("{}", a.describe());
  println!("{}", base_cheer(&a));
  println!("{:?}", a);

  println!("{}", analyze(&a));
  println!("{:?}", a);
}

fn analyze(a: &dyn Base) -> String {
  a.describe()
}

fn calculate<T, U>(a: T, b: T, c: U) -> T
where
  T: std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
  U: std::fmt::Debug,
{
  println!("c {:?}", c);
  a - b
}

fn base_cheer<V>(v: &V) -> String
where
  V: Base,
{
  let vv = v.describe();
  format!("{} {}", "Hello", vv)
}
