use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct MyType(i32);

impl Add<MyType> for MyType {
    type Output = MyType;

    fn add(self, rhs: MyType) -> MyType {
        MyType(self.0 + rhs.0)
    }
}

fn main() {
    let a = MyType(5);
    let b = MyType(3);
    let result = a + b;
    println!("Result: {:?}", result);
}
