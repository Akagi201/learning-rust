#[cfg(feature = "a")]
extern crate bar;
#[cfg(feature = "b")]
extern crate foo;

#[cfg(feature = "a")]
pub fn run() {
    // do something with crate bar
    println!("执行 a");
}

#[cfg(feature = "b")]
pub fn run() {
    // do something with crate foo
    println!("执行 b");
}
