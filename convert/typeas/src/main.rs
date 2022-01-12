use std::mem;

fn main() {
    let a: i8 = -10;
    let b = a as u8;
    println!("a={}, b={}", a, b);

    unsafe {
        let a = [0u8, 1u8, 0u8, 0u8];
        let b = mem::transmute::<[u8; 4], u32>(a);
        println!("{}", b); // 256
        // Or, more concisely:
        let c: u32 = mem::transmute(a);
        println!("{}", c); // 256
    }
}
