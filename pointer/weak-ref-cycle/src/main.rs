#![allow(unused)]
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct People {
    data: usize,
    parent: RefCell<Weak<People>>,
    children: RefCell<Vec<Rc<People>>>,
}

fn main() {
    let tom_son = Rc::new(People {
        data: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("tom_son parent = {:?} ", tom_son.parent.borrow());
    println!("tom_son parent = {:?}", tom_son.parent.borrow().upgrade());
    println!(
        "tom_son rc count={}, weak count={}",
        Rc::strong_count(&tom_son),
        Rc::weak_count(&tom_son)
    );

    let tom = Rc::new(People {
        data: 100,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&tom_son)]),
    });
    // tom 为 tom_son 的 parent, tom 从 Rc 转换为 Weak
    *tom_son.parent.borrow_mut() = Rc::downgrade(&tom);
    println!("tom_son parent = {:?}", tom_son.parent.borrow().upgrade());
    println!(
        "tom_son rc count={}, weak count={}",
        Rc::strong_count(&tom_son),
        Rc::weak_count(&tom_son)
    );
}
