pub fn run() {
  let s = "hello";
  read_str(s);

  let ss = String::from("Hey");
  read_into_string(ss);

  let ss2 = String::from("Wow");
  read_string(&ss2);
}

fn read_str(s: &str) {
  println!("{}", s);
}

fn read_into_string(s: String) {
  println!("{}", s);
}

fn read_string(s: &String) {
  println!("{}", s);
}
