
fn run() {

  //* While Loop
  let mut x = 0;

  while x < 3 {
      println!("{}", x);
      x += 1;
  }

  for i in 1..10 {
      println!("Hiii {}", i);
  }

  //* For in Loop
  let fruits = ["apple", "orange", "tomato"];
  for fruit in fruits.iter() {
      println!("{}", fruit);
  }


  for i in 1..3 {
      println!("Errr {}", i);
  }

  //* Loop
  let mut count = 1;

  loop {
      println!("{}", "Yeah");

      if count == 3 {
          break;
      }
      
      count += 1;
  }
}