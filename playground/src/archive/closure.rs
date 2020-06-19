fn main() {
  let mut a = vec![1, 2, 3, 5];
  let res = a.iter().any(|&e| e != 2);
  println!("{}", res);

  println!("{:?}", a);

  let mut ok4 = || {
    a.pop();
  };
  println!("{:?}", ok4());
  println!("{:?}", a);

  let x = 1;
  let ok1 = || &x + 2;
  println!("{}", ok1());
  println!("{}", x);

  let mut y = 5;
  let mut ok2 = || {
    y += 2;
    y
  };
  println!("ok2 {}", ok2());
  println!("y {}", y);
}
