fn main() {
    let a = 1;
    let rawp = &a as *const i32;

    unsafe {
        println!("rawp is {}", *rawp);
    }
}
