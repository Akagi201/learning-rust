extern crate rlp;

fn main() {
    {
        // decode
        let data = vec![0x83, b'c', b'a', b't'];
        let animal: String = rlp::decode(&data).expect("could not decode");
        assert_eq!(animal, "cat".to_owned());
    }

    {
        // encode
        let animal = "cat";
        let out = rlp::encode(&animal).to_vec();
        assert_eq!(out, vec![0x83, b'c', b'a', b't']);
    }
}
