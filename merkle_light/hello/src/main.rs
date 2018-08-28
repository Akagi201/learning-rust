extern crate crypto;
extern crate merkle_light;

use std::fmt;
use std::hash::Hasher;
use std::iter::FromIterator;
use crypto::sha3::{Sha3, Sha3Mode};
use crypto::digest::Digest;
use merkle_light::hash::{Algorithm, Hashable};
use merkle_light::merkle::MerkleTree;

pub struct ExampleAlgorithm(Sha3);

impl ExampleAlgorithm {
    pub fn new() -> ExampleAlgorithm {
        ExampleAlgorithm(Sha3::new(Sha3Mode::Sha3_256))
    }
}

impl Default for ExampleAlgorithm {
    fn default() -> ExampleAlgorithm {
        ExampleAlgorithm::new()
    }
}

impl Hasher for ExampleAlgorithm {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.input(msg)
    }

    #[inline]
    fn finish(&self) -> u64 {
        unimplemented!()
    }
}

impl Algorithm<[u8; 32]> for ExampleAlgorithm {
    #[inline]
    fn hash(&mut self) -> [u8; 32] {
        let mut h = [0u8; 32];
        self.0.result(&mut h);
        h
    }

    #[inline]
    fn reset(&mut self) {
        self.0.reset();
    }
}

fn main() {
    let mut h1 = [0u8; 32];
    let mut h2 = [0u8; 32];
    let mut h3 = [0u8; 32];
    h1[0] = 0x11;
    h2[0] = 0x22;
    h3[0] = 0x33;

    let t: MerkleTree<[u8; 32], ExampleAlgorithm> = MerkleTree::from_iter(vec![h1, h2, h3]);
    println!("{:?}", t.root());
    println!("{:?}", t.len());
    println!("{:?}", t.height());
}