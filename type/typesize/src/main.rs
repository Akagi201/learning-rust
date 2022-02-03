// 类型大小
// 编译期为变量/值分配地址，Rust 内存首先由编译器来分配，Rust 代码编译为 LLVM IR 中间语言，其中携带了内存分配的信息。

// 零大小类型：不分配内存空间，没有地址，在运行时不占用内存空间，提高性能
enum Void {} // 空枚举
struct Foo; // 单元结构体
#[allow(dead_code)]
struct Bar {
    foo: Foo,
    qux: (), // 单元类型
    baz: [u8; 0], // 空数组
}

// 一个结构体实现多个 trait 时
struct S(i32);
trait A {
    fn test(&self, i: i32) {
        println!("A: {}", i);
    }
}

trait B {
    fn test(&self, i: i32) {
        println!("B: {}", i);
    }
}

impl A for S{}
impl B for S{}

fn main() {
    // Rust 中大部分类型都可以在编译期确定大小
    println!("bool: {}", std::mem::size_of::<bool>());
    println!("u8: {}", std::mem::size_of::<u8>());
    println!("u16: {}", std::mem::size_of::<u16>());
    println!("u32: {}", std::mem::size_of::<u32>());
    println!("u64: {}", std::mem::size_of::<u64>());
    println!("usize: {}", std::mem::size_of::<usize>());
    println!("i8: {}", std::mem::size_of::<i8>());
    println!("i16: {}", std::mem::size_of::<i16>());
    println!("i32: {}", std::mem::size_of::<i32>());
    println!("i64: {}", std::mem::size_of::<i64>());
    println!("isize: {}", std::mem::size_of::<isize>());
    println!("char: {}", std::mem::size_of::<char>());
    println!("f32: {}", std::mem::size_of::<f32>());
    println!("f64: {}", std::mem::size_of::<f64>());
    println!("&str: {}", std::mem::size_of::<&str>()); // 胖指针
    println!("&[u32]: {}", std::mem::size_of::<&[u32]>()); // 胖指针
    println!("&[u32; 5]: {}", std::mem::size_of::<&[u32; 5]>()); // 普通指针
    println!("enum Void: {}", std::mem::size_of::<Void>());
    println!("(): {}", std::mem::size_of::<()>());
    println!("Foo: {}", std::mem::size_of::<Foo>());
    println!("Bar: {}", std::mem::size_of::<Bar>());
    println!("[(); 10]: {}", std::mem::size_of::<[(); 10]>());

    // 动态大小类型，DST
    // 对于无法确定大小的类型，只能在运行时分配，使用指针关联。
    // 胖指针：内存地址 + 长度信息
    // &str 字符串切片就是一个胖指针
    let str = "Hello world!";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:p}", ptr);
    println!("{:?}", len);

    // 零大小类型典型应用，有些场合，有些场景下，只需要迭代次数的场合中。
    // 利用零大小类型来实现，提升性能，Vec 内部迭代器中会针对零大小类型做一些优化。
    let v = vec![(); 10];
    for i in v {
        println!("{:?}", i);
    }

    // 无歧义完全限定语法
    let s = S(1);
    <S as A>::test(&s, 2); // 结合 as 关键字可以避免歧义
    <S as B>::test(&s, 3);
}
