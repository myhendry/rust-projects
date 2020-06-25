fn run() {
  err1::run();

  // let mut kk1 = String::from("yeahhhh");
  // let kk2 = &kk1;
  // kk1.push_str("ahhhh");
  // println!("{}", kk2);
  // println!("{}", kk1);

  let mut ww = String::from("zzzzz");
  let ww1 = &ww;
  println!("ww1 {}", ww1);
  println!("{}", ww1);
  ww.push_str("aaaaa");
  println!("ww {}", ww);

  let mut s = String::from("ok");
  let r = &s;
  println!("r {}", r);
  s.push_str(", ko"); // mutable borrow occurs here
  println!("{}", s);

  let ss1 = String::from("ok");
  let ss2 = using_ref(&ss1);
  println!("ss2 {}", ss2);
  let cc1 = get_first_value(&ss1, 1);
  println!("cc1 {:?}", cc1);
  if let Some(x) = cc1 {
    println!("cc1 x {}", x);
  }
  let mut ss3 = String::from("hello");
  change_val(&mut ss3, "wow");
  println!("ss3 {}", ss3);
  let ss31 = get_first_value(&ss3, 2);
  println!("ss31 {:?}", ss31);
  println!("{}", ss3);

  //let mut zz1 = String::from("wowowwwow");
  //let zz2 = &mut zz1;
  //let zz3 = &mut zz1; //*! cannot borrow `zz1` as mutable more than once at a time
  // println!("{} {}", zz2, zz3);

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
