extern crate blake2;

use blake2::{Blake2b, Digest};

fn main() {
    {
        // case 1
        let mut hasher = Blake2b::new();
        let data = b"Hello world!";
        hasher.input(data);
        // `input` can be called repeatedly
        hasher.input("String data".as_bytes());
        // Note that calling `result()` consumes hasher
        let hash = hasher.result();
        println!("Result: {:x}", hash);
    }

    {
        // case 2
        let hash = Blake2b::digest(b"Hello world!String data");
        println!("Result: {:x}", hash);
    }
}
