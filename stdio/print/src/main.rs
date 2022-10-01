use std::io;

fn main() {
    println!("请输入一个字符串: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取失败");
    println!("您输入的字符串是: {}", input);
}
