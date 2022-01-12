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
}
