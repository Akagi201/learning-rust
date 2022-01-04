fn main() {
    {
        // 数组实现了 Copy trait，在栈上分配的数组可以被复制
        let mut arr = [1,2,3,4];
        let mut arr2 = arr; // arr2 是 arr 的拷贝并不是原有数据
        arr[0] = 100;
        arr2[0] = 200;
        println!("arr: {:?}, arr2: {:?}", arr, arr2);
    }
    {
        // 堆上分配的数组不能被复制
        let arr = Box::new([1,2,3,4]);
        let arr2 = arr; // arr 所有权转移给 arr2
        // println!("arr: {:?}", arr); // 失败
        println!("arr2: {:?}", arr2);
    }

    {
        let arr = Box::new([1,2,3,4]);
        {
            let arr2 = arr;
            println!("arr2: {:?}", arr2);
        }
        // println!("arr: {:?}", arr);
    }

    {
        let mut arr = Box::new([1,2,3,4]);
        arr = print_arr(arr);
        println!("arr: {:?}", arr);
    }

    {
        let arr = Box::new([1,2,3,4]);
        print_arr1(&arr);
        println!("arr: {:?}", arr);
    }

    {
        let arr = Box::new([1,2,3,4]);
        print_arr2(&arr);
        println!("arr: {:?}", arr);
    }
}

fn print_arr(arr: Box<[i32;4]>) -> Box<[i32;4]> {
    println!("arr: {:?}", arr);
    arr
}

fn print_arr1(arr: &Box<[i32;4]>) {
    println!("arr: {:?}", arr);
}

fn print_arr2(arr: &[i32;4]) {
    println!("arr: {:?}", arr);
}
