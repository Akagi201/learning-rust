#![allow(unused)]
use std::{
    sync::{Arc, Mutex, Weak},
    thread,
};

struct Owner {
    name: String,
    dogs: Mutex<Vec<Weak<Dog>>>,
}

struct Dog {
    name: String,
    owner: Arc<Owner>,
}

fn main() {
    let some_one = Arc::new(Owner {
        name: "tom".to_string(),
        dogs: Mutex::new(vec![]),
    });
    let yellow_dog = Arc::new(Dog {
        name: "yellow dog".to_string(),
        owner: Arc::clone(&some_one),
    });
    let black_dog = Arc::new(Dog {
        name: "black dog".to_string(),
        owner: Arc::clone(&some_one),
    });

    for i in 0..10 {
        let some_one = Arc::clone(&some_one);
        let yellow_dog = Arc::clone(&yellow_dog);
        let black_dog = Arc::clone(&black_dog);
        let join_handle = thread::spawn(move || {
            let mut guard = some_one.dogs.lock().unwrap();
            guard.push(Arc::downgrade(&yellow_dog));
            guard.push(Arc::downgrade(&black_dog));
            println!("owner first dog {}", guard[0].upgrade().unwrap().name);
            println!("owner second dog {}", guard[1].upgrade().unwrap().name);

            println!("Thread {i} End");
        });
        join_handle.join().unwrap();
    }
}
