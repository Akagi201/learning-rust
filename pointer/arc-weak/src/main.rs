#![allow(unused)]
use std::{
    sync::{Arc, Weak},
    thread,
};

struct Owner {
    name: String,
    dogs: Vec<Weak<Dog>>,
}

struct Dog {
    owner: Arc<Owner>,
}

fn main() {
    let some_one = Arc::new(Owner {
        name: "tom".to_string(),
        dogs: vec![],
    });

    for i in 0..10 {
        let some_one = Arc::clone(&some_one);
        let join_handle = thread::spawn(move || {
            let yellow_dog = Arc::new(Dog {
                owner: Arc::clone(&some_one),
            });
            let black_dog = Arc::new(Dog {
                owner: Arc::clone(&some_one),
            });
            println!("yellow dog owner: {}", yellow_dog.owner.name);
            println!("black dog owner: {}", black_dog.owner.name);
            println!("Thread {i} End");
        });
        join_handle.join().unwrap();
    }
}
