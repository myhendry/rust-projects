use colored::*;

trait Talk {
  fn chat(&self, msg: &str) -> String;
}

#[derive(Debug)]
struct Human {
  name: String,
  age: u32,
}

impl Talk for Human {
  fn chat(&self, msg: &str) -> String {
    format!("{} {}", &self.name, msg)
  }
}

impl Human {
  fn speak(&self) -> String {
    format!("{}", &self.name)
  }
}

#[derive(Debug)]
struct Animal {
  nickname: String,
  age: u32,
}

impl Talk for Animal {
  fn chat(&self, msg: &str) -> String {
    format!("{}", msg)
  }
}

#[derive(Debug)]
enum Character {
  Animal { nickname: String, age: u32 },
  Human { name: String, age: u32 },
}

fn run() {
  let p = Character::Animal {
    nickname: "Yah".to_string(),
    age: 20,
  };

  println!("{:?}", p);

  let h = Character::Human {
    name: "Johnny".to_string(),
    age: 22,
  };

  println!("{:?}", h);

  let h2 = Human {
    name: "Tom".to_string(),
    age: 18,
  };

  println!("{}", h2.chat("Yeahhhh"));

  let mut t = (22, 33., 'x', "Hey");
  t.1 = 88.;
  println!("{:?}", t);
  println!("{}", 1);
  let s = "Apple";
  println!("{}", s.red());
  let s2 = "Orange".yellow().bold().on_blue();
  println!("{}", s2);
}
