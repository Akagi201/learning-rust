use std::io::{self, Write};

fn main() {
    print!("请输入一个字符串: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取失败");
    println!("您输入的字符串是: {}", input);
}
