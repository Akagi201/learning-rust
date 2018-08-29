extern crate rlp;

fn main() {
    let data = vec![0x83, b'c', b'a', b't'];
    let animal: String = rlp::decode(&data).expect("could not decode");
    assert_eq!(animal, "cat".to_owned());
}