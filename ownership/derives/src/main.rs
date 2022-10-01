#[derive(Debug)]
struct Foo {
    a: i32,
    b: bool,
}

impl Copy for Foo {}
impl Clone for Foo {
    fn clone(&self) -> Foo {
        Foo {
            a: self.a,
            b: self.b,
        }
    }
}

fn main() {
    let x = Foo { a: 100, b: true };
    let mut y = x;
    y.b = false;

    println!("{:?}", x);
    println!("{:?}", y);
}
