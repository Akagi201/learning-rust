use std::cell::{Cell, RefCell};

fn main() {
    // &str 是 Copy, String 不是 Copy
    let c = Cell::new("hello1");
    let c1 = c.get();
    c.set("hello2");
    let c2 = c.get();
    println!("{c1} {c2}");

    // Copy
    // let c = Cell::new(String::from("rust"));
    // println!("{c:?}");
    // let c3 = c.get();
    // println!("{c3}");

    let r = RefCell::new(String::from("hello"));
    // let r1 = r.borrow();
    // println!("{r1}");

    let r2 = r.borrow_mut();
    println!("{r2}");
}
