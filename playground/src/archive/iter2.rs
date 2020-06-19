#[derive(Debug)]
enum Gender {
  Male(i32),
  Female(i32),
}

impl Gender {
  fn style(&self) {
    match *self {
      Gender::Male(x) => println!("{} {}", "Male", x),
      Gender::Female(y) => println!("{} {}", "Female", y),
    }
  }
}

#[derive(Debug)]
struct A<'a, T> {
  v: &'a Vec<i32>,
  x: T,
  y: T,
  g: Gender,
}

impl<'a, T> A<'a, T>
where
  T: std::cmp::PartialOrd,
{
  fn new(v: &'a Vec<i32>, x: T, y: T, g: Gender) -> Self {
    A { v, x, y, g }
  }

  fn add(&self) -> T
  where
    T: std::ops::Add<Output = T> + Copy,
  {
    self.x + self.y
  }

  fn mul(&self) -> T
  where
    T: std::ops::Mul<Output = T> + Copy,
  {
    self.x * self.y
  }

  fn cycle(&self) -> Vec<i32> {
    self.v.iter().map(|x| x * 10).collect()
  }

  fn choice(&self, arg1: T) -> Option<T> {
    if self.x > arg1 {
      Some(arg1)
    } else {
      None
    }
  }
}

fn run() {
  let v = vec![12, 20, 50];
  let a = A::new(&v, 32, 20, Gender::Male(20));
  println!("{:?}", a);
  println!("{}", a.x);
  println!("{}", a.y);
  println!("{}", a.add());

  let b = A::new(&v, 100, 200, Gender::Female(30));
  println!("{}", b.add());
  println!("{}", b.mul());
  let v1 = b.cycle();
  println!("{:?}", v1);
  println!("{:?}", b);
  println!("{:?}", a.g);
  a.g.style();
  println!("{:?}", a);
  let a2 = a.choice(20);
  if let Some(q) = a2 {
    println!("q {}", q);
  }

  let mut v2 = vec![10, 20, 30];
  mut_rent(&mut v2); //* borrowed as mutable borrow
  println!("{:?}", &v2);
  println!("{:?}", v2);
  rent(&v2); //* borrowed as immutable borrow
  println!("{:?}", &v2);
  println!("{:?}", v2);
  rent(&v2);
  mut_rent(&mut v2);
  rent(&v2);
  //*! own_rent(v2); //* move occurs here
  //*! println!("{:?}", v2); //* error! move occurs

  let mut qq = Vec::new();
  qq.push(100);
  qq.push(200);
  for x in &qq {
    println!("x {}", x);
  }

  let mut zz = vec![300, 400, 500];
  zz.push(800);
  for z in &zz {
    println!("z {}", z);
  }

  let vo = vec![30, 50, 70, 80];
  for uu in vo.iter() {
    println!("uu {}", uu);
  }
  println!("vo 1 {:?}", vo);

  for uu in &vo {
    println!("{}", uu);
  }
  println!("vo 2 {:?}", vo);

  for uu in vo.iter() {
    println!("{}", uu);
  }
  println!("vo 3 {:?}", vo);

  let mut rr = String::from("Hello");
  let kk = &rr;
  println!("{}", rr);
  println!("{}", kk);
  let ww = &mut rr;
  ww.push_str("Hey");
  println!("{}", ww);
  println!("{}", rr);
  let yy = &rr;
  println!("{}", yy);
  println!("{}", rr);
  let oo = &mut rr;
  oo.push_str("aaaahhh");
  println!("{}", oo);
  println!("{}", rr);
}

fn rent(v: &Vec<i32>) {
  println!("{:?}", v);
}

fn mut_rent(v: &mut Vec<i32>) {
  v.push(300);
  println!("{:?}", v);
}

// fn own_rent(v: Vec<i32>) {
//   println!("{:?}", v);
// }
