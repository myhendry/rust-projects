
use rand::prelude::*;

//* closure as output param 8:18

//* closure + struct with generic
struct A<F: Fn(i32) -> i32> {
  f: F,
}

//* closure with generic
fn add1<F>(f: F)
where
  F: Fn(),
{
  f()
}

fn add2<F>(f: F) -> i32
where
  F: Fn(i32) -> i32,
{
  f(3)
}

//* function which takes generic closure which uses integer type
fn check<F>(f: F) -> i32
where
  F: Fn(i32) -> i32,
{
  let ll = 22;
  f(ll)
}

//todo function which takes generic closure which uses type on heap
// fn mix_h<F>(f: F) -> Box<i32>
// where
//   F: Fn(Box<i32>) -> Box<i32>,
// {
//   let kk = Box::new(66);
//   f(kk)
// }

#[derive(Debug)]
enum List {
  Cons(i32, Box<List>),
  End,
}

use List::Cons;
use List::End;

fn run() {
  //* smart pointers
  let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));
  println!("l {:?}", l);

  //* closures can be used with functions, structs (with generics), as output param (make as concrete types

  //* simple closures
  let vv = 32;
  let vvf = |x| x + vv;
  println!("vvf {}", vvf(vv));
  println!("{}", vv);

  let mut rr = 20;
  let mut rrf = || {
    rr += 22;
    println!("rr1 {}", rr);
  };
  rrf();
  rrf();
  println!("rr2 {}", rr);

  let oof = || println!("{}", "Hello closure");
  oof();

  //* closure + function with generic
  let cc1 = || println!("{}", "Hello cc1");
  let cc2 = |x| {
    let kk = x + 2;
    println!("kk {}", kk);
    kk
  };
  add1(cc1);
  add2(cc2);

  //* closure + struct with generic
  let _aa = A { f: cc2 };

  let _uu = Box::new(20);
  let oo = 3;
  let ff = |x| x + oo;
  println!("ff res {}", ff(6));
  println!("oo {}", oo);

  let qq = check(ff);
  println!("qq {}", qq);

  let mut rng = rand::thread_rng();
  let nums: Vec<i32> = (0..20).map(|_x| rng.gen_range(1, 11)).collect();
  println!("{:?}", nums);
  println!("{:?}", rng);

  let mut aa = String::from("apple");
  aa.push('z');
  println!("aa {}", aa);

  let xx: u8 = random();
  println!("xx {}", xx);

  //* iter(), into_iter(), iter_mut()
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  for val in v1_iter {
    println!("{}", val);
  }
  println!("v1 {:?}", v1);

  let v2 = vec!["Apple", "Orange", "Banana"];
  for val in v2.iter() {
    println!("{}", val);
  }
  println!("v2 {:?}", v2);

  let v3 = vec!["John".to_string(), "Aaron".to_string(), "Alvin".to_string()];
  for val in v3.iter() {
    println!("{}", val);
  }
  println!("v3 {:?}", v3);

  let v4 = vec![Box::new(10), Box::new(20), Box::new(30)];
  for val in v4.iter() {
    println!("{}", val);
  }
  println!("v4 {:?}", v4);

  let v5 = vec![10, 20, 30];
  let v5i: Vec<_> = v5.iter().map(|x| x + 1).collect();
  println!("v5i {:?}", v5i);
}
