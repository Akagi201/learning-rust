#![allow(unused)]
fn main() {
    // 指向不可变
    let x: usize = 1;
    let raw_pointer1 = &x as *const usize;
    let raw_pointer2: *const usize = &x;

    // 指向可变
    let mut y: usize = 2;
    let raw_mut_pointer1 = &mut y as *mut usize;
    let raw_mut_pointer2: *mut usize = &mut y;

    let some_usize1 = unsafe { *raw_pointer1 };
    println!("{some_usize1}");

    let some_usize2 = unsafe { *raw_pointer2 };
    println!("{some_usize2}");

    let some_mut_usize1 = unsafe { *raw_mut_pointer1 };
    println!("{some_mut_usize1}");

    let some_mut_usize2 = unsafe { *raw_mut_pointer2 };
    println!("{some_mut_usize2}");
}
