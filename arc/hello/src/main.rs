
use std::{time, thread, sync::{Arc, Mutex}};

struct Hello {
    v: i32,
}

impl Hello {
    fn say_hello(&self) {
        let id = thread::current().id();
        println!("{:?} {:p} => {}", id, self, self.v);
    }
    fn change(& mut self) {
        self.v += 10;
    }
}

fn main() {
    let h = Arc::new(Mutex::new(Hello{v:10}));
    for _ in 0..10 {
        let h = h.clone();
        thread::spawn(move || {
            let mut l = h.lock().unwrap();
            l.say_hello();
            l.change();
        });
    }
    thread::sleep(time::Duration::from_secs(1));
}