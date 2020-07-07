#[derive(Debug)]
enum Size {
  Small(u32),
  Large(u32),
}

impl Size {
  fn exec(&self) -> u32 {
    match *self {
      Size::Small(x) => x,
      Size::Large(x) => x,
    }
  }
}

trait Shape {
  fn read(&self, n: u32) -> u32;
}

#[derive(Debug)]
struct Style {
  x: String,
}

impl Style {
  fn new(arg1: &str) -> Self {
    Self {
      x: String::from(arg1),
    }
  }
}

impl Shape for Style {
  fn read(&self, n: u32) -> u32 {
    n + 3000
  }
}

struct Point {
  x: u32,
  y: u32,
  z: Size,
}

impl Point {
  fn new(x: u32, y: u32, z: Size) -> Self {
    Self { x, y, z }
  }

  fn calc(&self) -> u32 {
    self.x + self.y
  }
}

impl Shape for Point {
  fn read(&self, n: u32) -> u32 {
    n + 1000
  }
}

fn main() {
  let p1 = Point::new(30, 25, Size::Small(22));
  println!("{}", p1.calc());
  println!("{}", p1.z.exec());

  let p2 = Point::new(80, 30, Size::Large(55));
  println!("{}", p2.calc());
  println!("{}", p2.z.exec());
  println!("{}", p2.read(33));

  let s1 = Style::new("hey");
  println!("{:?}", s1);
  println!("{}", s1.read(33));

  println!("{}", scrape(&s1, 880));
  println!("{}", scrape(&p1, 550));
}

fn scrape<T>(x: &T, n: u32) -> u32
where
  T: Shape,
{
  x.read(n)
}
