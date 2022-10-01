struct Person {
    name: String,
}

impl Person {
    fn new(n: &str) -> Person {
        Person {
            name: n.to_string(),
        }
    }

    fn greeting(&self) {
        println!("{} say hello.", self.name);
    }
}

fn main() {
    let peter = Person::new("Peter");
    peter.greeting();
}
