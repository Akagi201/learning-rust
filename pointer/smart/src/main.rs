use std::ops::Deref;
use std::rc::Rc;

// ConsList 每一项包含两个元素：当前项和下一项
//                           结束项
// ConsList(0, ConsList(1, ConsList(2, Nil)))
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 0 -> 1 \
//        | -> 4
// 2 -> 3 /
enum List1 {
    Cons(i32, Rc<List1>),
    Nil,
}

// & -> Box -> MyBox -> String/Vec Rc -> Arc -> ReCell
#[allow(unused_variables)]
fn main() {
    let a = 10;
    let ra = &a;
    let av = *ra;

    let b = Box::new(20);
    let bv = *b;

    let c = MyBox::new(30);
    let cv = *c;

    let list = List::Cons(
        0,
        Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))),
    );

    let four = Rc::new(List1::Cons(4, Rc::new(List1::Nil)));
    // let zero_one = List1::Cons(0, Rc::new(List1::Cons(1, four.clone())));
    let zero_one = List1::Cons(0, Rc::new(List1::Cons(1, Rc::clone(&four))));
    let two_three = List1::Cons(2, Rc::new(List1::Cons(3, four)));
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
