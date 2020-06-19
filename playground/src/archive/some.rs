fn run() {
  let x = calc(5, 2);
  if let Some(y) = x {
    println!("{}", y);
  }
  let y = calc(8, 6).unwrap();
  println!("{}", y);

  let z = calc(20, 8).unwrap();
  println!("{}", z);
}

fn calc(num1: i32, num2: i32) -> Option<f64> {
  if num2 as f64 == 0. {
    None
  } else {
    Some(num1 as f64 / num2 as f64)
  }
}
