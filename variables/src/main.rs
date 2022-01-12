#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    {
        // let a = [3;5];
        // println!("{:?}", a[7]);
        // let (tup, _, _) = (1,2,3);
        // println!("{}", tup);
        // let x = 5;
        // let y = x;
        // let y = x.clone();
        // println!("{}, {}",x, y)
        let s1 = "123";
        let s2 = s1;
        println!("{} {}", s1, s2);
    }

    {
        let h: &str = "hello world"; //这个 h 是一个只读的字符串切片的引用。
        let mut mh: &str = "hello world"; // 这个 mh 是一个只读的字符串切片的引用，但是这个引用的指向可以被修改。

        let a = 132i32; // 这个 a 是一个 i32 的只读变量，从这个变量上，只能生出只读引用。
        let mut ma = 132i32; // 这个 ma 是一个 i32 的可变变量，从这个变量上，既能生出只读引用，也能生出可变引用。
        let b: &i32 = &a; // 这个 b 是对 a 的只读引用，只能指向 a，不能修改 b 的指向了。
        let mut mb: &i32 = &a; // 这个 mb 是对 a 的只读引用， 但是本身的 mb 的指向是可以修改的。

        let c: &mut i32 = &mut ma; //这个 c 是对 ma 的可变引用，可以修改 ma 的值，但是它只能指向 ma，不能修改 c 的指向了。
        let mut mc: &mut i32 = &mut ma; //这个 mc 是对 ma 的可变引用，不但可以修改 ma 的值，其本身的 mc 的指向也是可以修改的。
    }
}
