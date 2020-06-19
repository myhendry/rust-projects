trait Human {
  fn talk(&self, s: String) -> String;
}

trait Base {
   fn young(&self) -> bool;
}

trait Mix {
   fn listen(&self) {
       println!("{}", "I'm mixed");
   }
}

#[derive(Debug)]
enum Education {
   PRIMARY(u32),
   _SECONDARY(u32),
   _JC(u32),
   _UNIVERSITY(u32),
}

#[derive(Debug)]
struct Man {
   name: String,
   age: u32,
   private: bool,
   education: Education
}

impl Man {
   fn new(name: String, age: u32, private: bool, education: Education) -> Self {
       Self {
           name,
           age,
           private,
           education,
       }
   }
}

impl Human for Man {
   fn talk(&self, s: String) -> String {
       format!("{} {}", s, String::from("Hello"))
   }
}

impl Base for Man {
   fn young(&self) -> bool {
       self.age < 20
   }
}
#[allow(dead_code)]
struct Animal {
   nick: String,
   age: u32,
}


impl Animal {
   fn new(nick: String, age: u32) -> Self {
       Self {
           nick, 
           age
       }
   }
}

#[allow(unused_variables)]
impl Human for Animal {
   fn talk(&self, s: String) -> String {
       format!("I am an animal and my name is {} {}", self.nick, s)
   }
}

impl Base for Animal {
   fn young(&self) -> bool {
       self.age < 20
   }
}

#[allow(unused_variables)]
#[allow(dead_code)]
struct Mammal {
   nick: String,
}

impl Mammal {
   fn new(nick: &str) -> Self {
       Self {
           nick: String::from(nick)
       }
   }
}

impl Mix for Mammal {
   fn listen(&self) {
       
   }
}

fn make_talk<T>(s: &T, arg1: &str) -> String
where T: Human
{
   s.talk(arg1.to_string())
}

fn discover<T>(s: &T) -> String
where T: Human + Base {
   let ss = s.talk(String::from("hey"));
   let zz = s.young();
   format!("{} {}", ss, zz)
}

fn challenge<T, U>(s: &T, u: &U) -> String
where T: Human + Base,
     U: Mix
{
   u.listen();
   let ss =s.talk(String::from("zzzzz"));
   ss
}
fn run() {
   let john = Man::new(String::from("John"), 32, true, Education::PRIMARY(3000));
   println!("{:?}", john);
   println!("{}", john.talk(String::from("Hey")));
   println!("{}", john.young());

   let am = Animal::new(String::from("Lucky"), 8);
   println!("{}", am.talk(String::from("Woof woof")));
   println!("{}", am.young());
   
   let z = make_talk(&am, "Hello");
   println!("make talks {}", z);

   println!("discover {}", discover(&am));

   let mm = Mammal::new("Mim");
   let xx = challenge(&am, &mm);
   println!("xxx {}", xx);
   
}