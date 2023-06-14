enum MyEnum {
    Type1(Type1),
    Type2(Type2),
    Type3(Type3),
}

struct Type1;
impl Type1 {
    fn method1(&self) {
        println!("Type1: method1");
    }
}

struct Type2;
impl Type2 {
    fn method2(&self) {
        println!("Type2: method2");
    }
}

struct Type3;
impl Type3 {
    fn method3(&self) {
        println!("Type3: method3");
    }
}

fn main() {
    let vec: Vec<MyEnum> = vec![
        MyEnum::Type1(Type1),
        MyEnum::Type2(Type2),
        MyEnum::Type3(Type3),
    ];

    for item in vec {
        match item {
            MyEnum::Type1(t) => t.method1(),
            MyEnum::Type2(t) => t.method2(),
            MyEnum::Type3(t) => t.method3(),
        }
    }
}
