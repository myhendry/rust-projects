fn run() {
  let mut w = String::from("apple");
  w.push_str("Hello");
  w.push('t');
  println!("{}", w);

  let z = vec![10, 20, 30];
  let z1 = &z[1];
  println!("z1 {}", z1);
  let z2 = &z[0..=1];
  println!("z2 {:?}", z2);
  println!("z {:?}", z);

  let y = vec![
    String::from("apple"),
    String::from("banana"),
    String::from("cucumber"),
  ];
  let y1: &String = &y[2];
  println!("y1 {}", y1);
  println!("y {:?}", y);
  let y2 = &y[0..=1];
  println!("y2 {:?}", y2);

  let s = String::from("happy");
  let s1: &str = &s[0..=3];
  println!("s1 {}", s1);
  println!("s {}", s);
  let s2 = &s[1..3];
  println!("s2 {}", s2);

  let r = vec![200, 300, 400];
  let r1 = &r[0..=1];
  println!("r1 {:?}", r1);
  println!("r {:?}", r);
  let r2: Vec<_> = r.iter().map(|x| x).collect();
  println!("r2 {:?}", r2);
}
