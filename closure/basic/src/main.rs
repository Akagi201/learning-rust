#[allow(unused)]
fn main() {
    {
        // case 1
        let plus_one = |x| x + 1;
        assert_eq!(2, plus_one(1));
    }

    {
        // case 2
        let plus_two = |x| {
            let mut result: i32 = x;
            result += 1;
            result += 1;

            result
        };

        assert_eq!(4, plus_two(2));
    }

    {
        // case 3
        let plus_one = |x: i32| -> i32 { x + 1 };
        assert_eq!(2, plus_one(1));
    }
    {
        // case 4
        fn plus_one_v1 (x: i32) -> i32 { x + 1 }
        let plus_one_v2 = |x: i32| -> i32 { x + 1 };
        let plus_one_v3 = |x: i32| x+1;
    }
}
