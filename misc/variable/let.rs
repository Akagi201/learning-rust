fn main() {
    // 变量绑定
    let a1 = 5;
    let a2:i32 = 5;
    assert_eq!(a1, a2);
    //let 绑定 整数变量默认类型推断是 i32

    // let b1:u32 = 5;
    // assert_eq!(a1, b1);
    //去掉上面的注释会报错，因为类型不匹配
    //errer: mismatched types

    // let 解构
    let (a, mut b): (bool,bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    //a 不可变绑定
    //a = false;

    //b 可变绑定
    b = true;
    assert_eq!(a, b);
}
