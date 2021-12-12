fn test<T>(f: T) where T:Fn() {
    f();
}

fn test1<T>(f: T, x: i32) where T:Fn(i32) {
    f(x);
}

fn test2<T>(mut f: T) where T:FnMut() {
    f();
}

fn test3<T>(f: T) where T:FnOnce() {
    f();
    // f();
}

fn test4<T>(f: T) where T:Fn() {
    f();
    f();
}

fn test5<T>(mut f: T) where T: FnMut() {
    f(); // 我需要 FnMut 但是你可以不改
    f();
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_closure1() -> impl Fn(i32) -> i32 {
    |x| x + 1 
}

fn returns_closure2() -> impl FnOnce() {
    let s = String::from("hello");
    move || println!("{}", s)
}

fn main() {
    // Fn
    let s = String::from("hello");
    let f = || {
        println!("{}", s);
    };
    test(f);

    let s1 = String::from("hello");
    let f1 = |x:i32| {
        println!("{}, {}", s1, x);
    };
    test1(f1, 1);

    // FnMut
    let mut s2 = String::from("hello");
    let f2 = || {
        s2.push_str("world");
    };
    test2(f2);

    // FnOnce
    let s3 = String::from("hello");
    let f3 = || {
        let _ = s3;
    };
    test3(f3);

    // move
    let s4 = String::from("hello");
    let f4 = move || {
        println!("{}", s4);
    };
    test4(f4);
    // println!("{}", s4);

    let s5 = String::from("hello");
    let f5 = || {
        println!("{}", s5); // 你可以不修改 s5
    };
    test5(f5);

    // let f6 = returns_closure();
    let f6 = returns_closure1();
    println!("res is {}", f6(11));

    let f7 = returns_closure2();
    f7();
}
