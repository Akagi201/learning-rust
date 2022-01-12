#![recursion_limit = "128"]
#[macro_use]
extern crate generic_array;
extern crate typenum;

use generic_array::GenericArray;
#[allow(unused_imports)]
use typenum::{U1, U3, U4, U97};

fn main() {
    {
        let ar = arr![u8; ];
        assert_eq!(format!("{:x}", ar), "");
    }

    {
        let ar = arr![u8; , ];
        assert_eq!(format!("{:x}", ar), "");
    }

    {
        let ar = arr![u8; 10, 20, 30];
        assert_eq!(format!("{:x}", ar), "0a141e");
    }

    {
        let ar = arr![u8; 10, 20, 30, ];
        assert_eq!(format!("{:x}", ar), "0a141e");
    }

    {
        let mut list97 = [0; 97];
        #[allow(clippy::needless_range_loop)]
        for i in 0..97 {
            list97[i] = i as i32;
        }
        let l: GenericArray<i32, U97> = GenericArray::clone_from_slice(&list97);
        assert_eq!(l[0], 0);
        assert_eq!(l[1], 1);
        assert_eq!(l[32], 32);
        assert_eq!(l[56], 56);
    }

    {
        let test: GenericArray<u32, U3> = arr![u32; 1, 2, 3];
        assert_eq!(test[1], 2);
    }
}
