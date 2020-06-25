mod example;

use example::err1;
//use example::sub1;

fn main() {
  err1::run();

  let s = String::from("hello world");
  let index_first_word = first_word(&s);
  println!("{}", index_first_word);
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }
  &s[..]
}
