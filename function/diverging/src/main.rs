fn main() {
    println!("hello");
    diverging();
    println!("world");
}

fn diverging() -> ! {
    panic!("This function will never return");
}