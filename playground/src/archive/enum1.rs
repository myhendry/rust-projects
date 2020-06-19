#[derive(Debug)]
#[allow(dead_code)]
#[allow(unused_variables)]

struct Person { 
    name: String,
    age: u32,
    role: Role,
}

impl Person {
    fn new(n: &str, age: u32, role: Role) -> Self {
    Self {
    name: Sring::from(n),
    age,
    role,
}
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Shape {
    Round(u32),
    Square(u32),
    Rectangle(u32, u32)
}
impl Shape {
    fn calculate_area(&self) -> u32 {
        match *self {
            Shape::Round(ref x) => (x * x),
            Shape::Square(ref x) => (x * x),
            Shape::Rectangle(ref x, ref y) => (x * y)
        }
    }
}
#[derive(Debug)]
#[allow(dead_code)]
enum Role {
    Active(String),
    Passive(String),
}

impl Role {
    fn match_role(&self) -> &String {
        match *self {
            Role::Active(ref s) => s,
            Role::Passive(ref s) => s,
        }
    }

    fn match_another(&self) -> String {
        match *self {
            Role::Active(ref s) => s.to_string(),
            Role::Passive(ref s) => s.to_string(),
        }
    }
}

fn run() {
    let person = Person::new("Hendry", 22, Role::Active(String::from("Yeah")));
    println!("{:?}", person);
    println!("{}", person.role.match_role());
    

    let circle = Shape::Round(20);
    println!("{:?}", circle.calculate_area());

    let role = Role::Active(String::from("I'm very active"));
    println!("{}", role.match_role());
    println!("{}", role.match_another());
}