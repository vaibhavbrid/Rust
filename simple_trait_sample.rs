// Your trait here
trait Describable {
    fn describe(&self) -> String;
}

// Do not modify
struct Person {
    name: String,
    age: u8,
}

// Your trait implementation for Person here
impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old.", self.name, self.age)
    }
}

// Do not modify
fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{}", person.describe());
}
