fn run() {
  let a = vec![String::from("Mandy"), String::from("Chris")];
  let a_iter = a.iter();
  for aa in a_iter {
    println!("aa {}", aa);
  }
  println!("a {:?}", a);

  let mut b = vec![1, 3, 5];
  let b_iter = b.iter_mut();
  for bb in b_iter {
    *bb += 10;
    println!("bb {}", bb);
  }
  println!("b {:?}", b);

  for c in 0..5 {
    println!("c {}", c);
  }

  let d = vec![11, 12, 13];
  let d1: Vec<_> = d.iter().map(|x| x * 2).collect();
  println!("d1 {:?}", d1);
  println!("d {:?}", d);

  let e = vec![String::from("Amy"), String::from("Alvin")];
  let e1: Vec<_> = e.iter().map(|x| x).collect();
  println!("e1 {:?}", e1);
  println!("e {:?}", e);

  let mut f = vec![String::from("USA"), String::from("Taiwan")];
  let f1: Vec<_> = f
    .iter_mut()
    .map(|x| {
      x.push_str("hello");
      x
    })
    .collect();
  println!("f1 {:?}", f1);
  println!("f {:?}", f);

  let g = vec![String::from("Malaysia"), String::from("Singapore")];
  let g1: Vec<_> = g
    .iter()
    .map(|x| {
      // let xx = String::from("hey") + &x;
      let xx = format!("{} {} {}", x, "hi".to_string(), x);
      xx
    })
    .collect();
  println!("g1 {:?}", g1);
  println!("g {:?}", g);

  //* using struct with Iterator trait
  let mut fib = Fibonacchi { curr: 1, next: 1 };
  for _ in 0..5 {
    println!("fib.curr {:?}", fib.curr);
    fib.next();
  }
  println!("fib {:?}", fib);

  let h = "&str";
  let i = "String".to_string();
  let j = String::from("Another_String");
  let l = format!("{} {}", h, i);
  println!("l {}", l);
  let k = format!("{} {}", j, i);
  println!("k {}", k);

  let m = vec![1, 5, 8, 12, 20];
  let mv: Vec<_> = m.iter().map(|x| x * 10).collect();
  println!("mv {:?}", mv);
  println!("m {:?}", m);

  let n = |x| x + 1;
  println!("n {}", n(2));

  let o = close(n);
  println!("o {}", o);

  //* 1 tensor youtube video on closures & iterators
  let p = (0..)
    .map(|n| n * n)
    .take_while(|n| n < &10000)
    .filter(|n| is_even(*n))
    .fold(0, |s, i| s + i);
  println!("p {}", p);

  //* 2 using structs with closures
  let sc = SC { f: n };
  let q = sc.f;
  let qr = q(32);
  println!("qr {}", qr);
}

//* 1 tensor youtube video on closures & iterators
fn is_even(n: i32) -> bool {
  n % 2 == 0
}

//* using functions with closures
fn close<T>(arg: T) -> i32
where
  T: Fn(i32) -> i32,
{
  arg(3)
}

//* 2 using structs with closures
#[derive(Debug)]
struct SC<F: Fn(i32) -> i32> {
  f: F,
}

//* using struct with Iterator trait
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
