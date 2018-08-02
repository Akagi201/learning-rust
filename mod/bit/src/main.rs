mod bits {
    fn pos(bit: u32) -> u32 {
        1 << bit
    }

    pub fn decimal(bit: u32) {
        println!("Bits decimal {}", pos(bit));
    }

    pub fn hex(bit: u32) {
        println!("Bits decimal 0x{:x}", pos(bit));
    }
}

fn main() {
    bits::decimal(8);
    bits::hex(8);
}

// Bits decimal 256
// Bits decimal 0x100
