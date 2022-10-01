fn main() {
    {
        // case 1
        let x: Vec<i32> = vec![1i32, 2, 3];
        let y = &x;
        println!("x={:?}, y={:?}", x, y);
    }

    {
        // case 2
        let mut x: i32 = 100;
        {
            let y: &mut i32 = &mut x;
            *y += 2;
        }
        println!("x={:?}", x);
    }

    {
        // case 3
        let x: Vec<i32> = vec![1i32, 2, 3];

        // 可同时有多个不可变借用
        let y = &x;
        let z = &x;
        let m = &x;

        // ok
        println!("{:?}, {:?}, {:?}, {:?}", x, y, z, m);
    }

    {
        // case 4
        // 源变量 x 可变性
        let mut x: Vec<i32> = vec![1i32, 2, 3];

        // 只能有一个可变借用
        let y = &mut x;
        // let z = &mut x; //错误
        y.push(100);

        // ok
        println!("{:?}", y);

        //错误，可变借用未释放，源变量不可访问
        // println!("{:?}", x);
    } //y在此处销毁

    {
        // case 5
        let mut x: Vec<i32> = vec![1i32, 2, 3];

        // 更新数组
        // push 中对数组进行了可变借用, 并在 push 函数退出时销毁这个借用
        x.push(10);

        {
            // 可变借用 1
            let mut y = &mut x;
            y.push(100);

            // 可变借用2, 注意: 此处是对 y 的借用, 不可再对x进行借用,
            // 因为 y 在此时依然存活.
            let z = &mut y;
            z.push(1000);

            println!("{:?}", z); // 打印: [1, 2, 3, 10, 100, 1000]
        } //y和z在此处被销毁，并释放借用。
          //访问x正常
        println!("{:?}", x); //打印: [1, 2, 3, 10, 100, 1000]
    }
}
