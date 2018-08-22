fn foo(x: i32) -> i32 {
    x * x
}

macro_rules! foo {
    ($x:ident) => {
        println!("{:?}", $x);
    }
}

fn main() {
    let a = 5;
    foo!(a);
    println!("{}", foo(a));
}