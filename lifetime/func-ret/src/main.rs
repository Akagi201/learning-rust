fn print_ret<'a>(s1: &str, s2: &'a str) -> &'a str {
    println!("{} {}", s1, s2);
    s2
}

fn print_ret1(s1: &str) -> &str {
    println!("{}", s1);
    s1
}

fn min<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x < y {
        x
    } else {
        y
    }
}

// 在实际调用的时候，才限定具体的类型，这个叫晚限定 (late bound)
// 并不是所有的生命周期函数都是 late bound，这里给出了两条规则
// 规则 1: 生命周期参数收到他必须超过的某个其他生命周期的限制. 'a: 'b
// 规则 2: 生命周期在函数签名之外声明，例如：在结构体的关联方法中，他可能来自结构本身。
#[allow(clippy::extra_unused_lifetimes)]
fn f<'a>() {} // late bound
#[allow(clippy::extra_unused_lifetimes)]
fn g<'a:'a>() {} // early bound

fn main() {
    if false {
        let some_str: String = "Some string".to_string();
        let other_str: String = "Other string".to_string();
        let s2 = print_ret(&some_str, &other_str);
        let s1 = print_ret1(&some_str);
        println!("{} {}", s1, s2);
    }

    {
        let p = 42;
        {
            let q = 10;
            let r = min(&p, &q);
            println!("Min is {}", r);
        }
    }

    // let pf = f::<'static> as fn();// 这里是声明，不是在实际调用的时候，提前绑定具体的生命周期，会报错。
    let pf = f as fn();
    let pg = g::<'static> as fn();
    assert_eq!(pf, pg);

}
