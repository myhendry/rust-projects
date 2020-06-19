fn run() {
  let v6 = vec![10, 20, 30];
  for x6 in v6 {
    println!("x6 {}", x6);
  }
  //* err - borrow of moved value
  //println!("v6 {}", v6);

  let v7 = vec![20, 30, 40];
  for x7 in &v7 {
    println!("x7 {}", x7);
  }
  println!("v7 {:?}", v7);

  //*! iter */
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  for x1 in v1_iter {
    println!("{}", x1 + 7);
  }
  //* err - borrow of moved value

  // let tt = v1_iter.next();
  println!("v1 {:?}", v1);
  for x1 in &v1 {
    println!("vx1 {}", x1);
  }
  println!("v1 {:?}", v1);
  let v1rr: Vec<i32> = v1.iter().map(|x| x * 10).collect();
  println!("v1rr {:?}", v1rr);
  println!("{:?}", v1);

  let v2 = vec![2, 3, 4];
  let mut v2_iter = v2.iter();
  let vr = v2_iter.next();
  if let Some(xx1) = vr {
    println!("xx1 {}", xx1);
  };
  let vr2 = v2_iter.next();
  if let Some(xx2) = vr2 {
    println!("xx2 {}", xx2);
  }
  println!("v2 {:?}", v2);

  //*! iter_mut */
  let mut v4 = vec![4, 5, 6];
  let v4_iter = v4.iter_mut();
  for x4 in v4_iter {
    let xr = *x4 + 3;
    println!("xr {}", xr);
  }
  println!("v4 {:?}", v4);

  //*! into_iter */
  let v3 = vec![3, 4, 5];
  let v3_iter = v3.into_iter();
  for x3 in v3_iter {
    println!("{}", x3);
  }
  //* err - borrow of moved value v3
  // println!("{:?}", v3);

  //*! sum */
  let vv4_iter = v4.iter();
  let vvr: i32 = vv4_iter.sum();
  println!("vvr {}", vvr);

  let v5 = vec![5, 6, 7];
  let v5r: Vec<i32> = v5.iter().map(|x| x + 3).collect();
  println!("{:?}", v5r);
  println!("{:?}", v5);

  //*! filter structs */
  let shoe1 = Shoe {
    size: 32,
    style: String::from("modern"),
  };
  let shoe2 = Shoe {
    size: 20,
    style: String::from("caribbean"),
  };
  let shoe3 = Shoe {
    size: 31,
    style: String::from("balinese"),
  };
  let vs = vec![shoe1, shoe2, shoe3];
  let vsr = shoes_in_my_size(vs, 30);
  println!("vsr {:?}", vsr);
}

#[derive(Debug)]
struct Shoe {
  size: i32,
  style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
  let shoes = shoes.into_iter().filter(|s| s.size >= shoe_size).collect();
  shoes
}
