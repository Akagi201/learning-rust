trait Animal {
    // Static method signature; 'Self' refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Cat {
    name: &'static str,
    age: u32,
}

struct Dog {
    name: &'static str,
}

impl Animal for Cat {
    // Self is the implementor type: Cat.
    fn new(name: &'static str) -> Self {
        Cat { name: name, age: 1 }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "meow"
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pause briefly... {}", self.name, self.noise());
    }
}

impl Animal for Dog {
    fn new(name: &'static str) -> Self {
        Dog { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "woof"
    }
}

fn main() {
    let c: Cat = Animal::new("Kate");
    println!("Cat's name is {} {}", c.name(), c.age);
    c.talk();

    let d: Dog = Animal::new("Spot");
    d.talk();
}