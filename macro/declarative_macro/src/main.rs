macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    ($name:expr, $greeting:expr) => {
        println!("{}, {}!", $greeting, $name);
    };
}

fn main() {
    greet!("Alice");
    greet!("Bob", "Good morning");
}
