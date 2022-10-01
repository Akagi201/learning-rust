fn main() {
    let a: u32 = u32::MAX;
    let b: u32 = 1;

    // println!("sum={}", a + b);

    let (r, is_overflow) = a.overflowing_add(b);
    println!("r={}, is_overflow={}", r, is_overflow);
}
