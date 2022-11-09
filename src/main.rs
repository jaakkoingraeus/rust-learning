use std::fmt;

struct Person {
    name: String,
    age: i32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, age: {}", self.name, self.age)
    }
}

impl Person {
    fn new(name: &str, age: i32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    let n = 69;

    let people = vec![Person::new("Jaakko", 26), Person::new("Henri", 29)];

    let last = people.last().expect("People array empty");

    println!("{}", last)
}
