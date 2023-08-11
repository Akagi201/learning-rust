use std::{thread, time::Duration};

fn my_thread() {
    println!("Thread {:?} is running", std::thread::current().id());
    thread::sleep(Duration::from_millis(1));
}

fn main() {
    let mut v = vec![];

    for _i in 1..10 {
        v.push(thread::spawn(|| {
            my_thread();
        }));
    }

    println!("main() waiting.");

    for child in v {
        match child.join() {
            Ok(_) => (),
            Err(why) => println!("Join failure {:?}", why),
        };
    }
}
