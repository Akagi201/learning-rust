trait MyTrait {
    fn method(&self);
}

struct Type1;
impl MyTrait for Type1 {
    fn method(&self) {
        println!("Type1: method");
    }
}

struct Type2;
impl MyTrait for Type2 {
    fn method(&self) {
        println!("Type2: method");
    }
}

struct Type3;
impl MyTrait for Type3 {
    fn method(&self) {
        println!("Type3: method");
    }
}

fn main() {
    let vec: Vec<Box<dyn MyTrait>> = vec![Box::new(Type1), Box::new(Type2), Box::new(Type3)];

    for item in vec {
        item.method();
    }
}
