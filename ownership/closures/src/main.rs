fn main() {
    {
        // case 1
        let x: i32 = 100;
        let some_closure = move |i: i32| i + x;
        let y = some_closure(2);
        println!("x={}, y={}", x, y);
    }

    {
        // case 2
        let mut x: String = String::from("abc");
        {
            let mut some_closure = |c: char| x.push(c);
            some_closure('d');
        }
        println!("x={:?}", x); // 成功打印：x="abcd"
    }
}
