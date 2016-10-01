// by Liigo, 2016-03-21

trait Run {
    fn run(&self);
}

impl Run for i32 {
    fn run(&self) {
        //println!("run i32 {}", *self);
    }
}

impl Run for f32 {
    fn run(&self) {
        //println!("run bool {}", *self);
    }
}

fn static_run<T: Run>(t: &T) {
    t.run();
}


fn dynamic_run(r: &Run) {
    r.run();
}

fn main() {
    static_run(&1);
    static_run(&1.0f32);
    dynamic_run(&1);
    dynamic_run(&1.0f32);
}
