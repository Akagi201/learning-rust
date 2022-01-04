
use std::ops::Deref;

// & -> Box -> MyBox -> String/Vec Rc -> Arc -> ReCell
fn main() {
    let a= 10;
    let ra = &a;
    let av = *ra;

    let b = Box::new(20);
    let bv = *b;

    let c = MyBox::new(30);
    let cv = *c;
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(v: T) -> MyBox<T> {
        MyBox(v)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}