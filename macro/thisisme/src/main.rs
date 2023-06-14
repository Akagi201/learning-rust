// import our crate
use whoami::WhoAmI;

#[derive(WhoAmI)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    println!("Hello, world!");
}
