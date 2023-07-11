#![allow(dead_code)]
trait Animal {
    fn eat(&self);
}

#[derive(Debug)]
struct Cat {
    children: Option<Box<Cat>>,
}

impl Animal for Cat {
    fn eat(&self) {
        println!("cat is eating");
    }
}

fn main() {
    let cat = Box::new(Cat { children: None });
    println!("{:?}", cat);

    let t: Box<dyn Animal> = Box::new(Cat {
        children: Some(cat),
    });
    t.eat();
}
