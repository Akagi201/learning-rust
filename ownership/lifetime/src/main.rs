fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if true {
        x
    } else {
        y
    }
}

fn main() {
    {
        // case 1
        let a = 100_i32;

        {
            let x = &a;
            println!("{}", x);
        } // x 作用域结束
    }

    {
        // case 2
    }
}
