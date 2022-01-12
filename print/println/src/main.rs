fn main() {
    // 可以使用额外的位置参数。
    println!("{0}{1}{0}", 4, 2);

    // 使用命名参数。
    println!("name={name} age={age}", name = r#"jack"#, age = 6);

    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 可以按指定宽度来右对齐文本。
    println!("{number:>width$}", number = 1, width = 6);

    // 在数字左边补 0.下面语句输出 "000001".
    println!("{number:>0width$}", number = 1, width = 6);
}
