fn use_str(s: &str) {
    println!("I am: {}", s);
}

fn main() {
    let x: &'static str = "hello";

    let mut y: String = x.to_string();
    println!("{}", y);
    y.push_str(", world");
    println!("{}", y);

    let s = "Hello".to_string();
    use_str(&*s);

    // UTF-8编码的字节数组转换成String
    // 存储在Vec里的一些字节
    let miao = vec![229, 150, 181];

    // 我们知道这些字节是合法的UTF-8编码字符串，所以直接unwrap()
    let meow = String::from_utf8(miao).unwrap();

    assert_eq!("喵", meow);
    println!("{}", meow);

    // index
    let x = "哎哟我去".to_string();
    for i in x.as_bytes() {
        print!("{} ", i);
    }

    println!("");

    for i in x.chars() {
        print!("{}", i);
    }

    x.chars().nth(2);
}
