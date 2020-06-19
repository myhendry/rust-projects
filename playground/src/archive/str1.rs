fn run() {
  let str1 = "Hello";
  let s1 = str1.to_string();
  println!("{}", str1);
  println!("{}", s1);
  println!("{}", str1);
  println!("{}", &s1);
  println!("{}", s1);
  println!("{}", str1);
  let str2 = "Day";
  let s2 = [str1, str2].concat();
  println!("{}", s2);
  println!("{}", str1);
  println!("{}", str2);
  let s3 = "Nice".to_string();
  let s4 = format!("{} {}", s3, str2);
  println!("{}", s4);
  println!("{}", s3);
  println!("{}", str2);
  let s5 = format!("{} {}", s2, s3);
  println!("{}", s5);
  println!("{}", s2);
  println!("{}", s3);
  let str3 = "How are you my friend?";
  let ss = &str3[0..5];
  println!("{}", ss);
  let ss2 = &str3[0..6];
  println!("{}", ss2);
  let ss3 = &str3[2..=7];
  println!("{}", ss3);
  let ccc1 = &str3.chars().nth(5);
  println!("{}", ccc1.unwrap());
  let s6 = [s3, s4];
  println!("{:?}", s6); //* return [String, String]
  let s7 = format!("{} {}", str3, str2);
  println!("s7 {}", s7);
  println!("{}", str3);
  println!("{}", str2);

  let zz = str3.chars().nth(5);
  println!("{:?}", zz);

  match zz {
    Some(x) => println!("zz {}", x),
    None => println!("zz {}", "Nothing found"),
  }

  if let Some(x) = zz {
    println!("if let zz {}", x);
  }

  let zzr = zz.unwrap();
  println!("zzr {}", zzr);
}
