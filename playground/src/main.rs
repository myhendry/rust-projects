mod example;

use example::err1;
//use example::sub1;

struct Point<T> {
  x: T,
  y: T,
}

impl Point<f32> {
  fn add_f32(&self) -> f32 {
    self.x + self.y
  }
}

impl<T> Point<T> {
  fn add(self) -> T
  where
    T: std::ops::Add<Output = T>,
  {
    self.x + self.y
  }
}

#[derive(Debug)]
struct Shape<T, U> {
  x: T,
  y: U,
}

impl<T, U> Shape<T, U> {
  fn mixup<V, W>(self, other: Shape<V, W>) -> Shape<T, W> {
    Shape {
      x: self.x,
      y: other.y,
    }
  }
}

fn largest<T>(list: &[T]) -> T
where
  T: std::cmp::PartialOrd + Copy, //* implement PartialOrd & Copy Trait
{
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn main() {
  err1::run();

  let sh = Shape { x: 30, y: 33.3 };
  let sh1 = sh.mixup(Shape { x: "Hello", y: 'c' });
  println!("{:?}", sh1);

  let p = Point { x: 32., y: 20. };
  println!("p add f32 {}", p.add_f32());
  let p1 = p.add();
  println!("p1 {}", p1);

  let p2 = Point { x: 300, y: 200 };
  println!("p2 add {}", p2.add());

  let p3 = Point { x: 27.2, y: 30.5 };
  println!("p3 {}", p3.add());

  // let p4 = Point { x: true, y: true };
  // println!("{}", p4.add());

  let v = vec![10, 20, 30, 150, 80];
  let v1 = largest(&v);
  println!("{}", v1);

  let v2 = vec!['a', 'c', 'b', 'l', 'z', 'p'];
  let v3 = largest(&v2);
  println!("{}", v3);
}
