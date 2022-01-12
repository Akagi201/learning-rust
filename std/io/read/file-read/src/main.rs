use std::io;
use std::fs::File;
use std::io::Read;

fn main() -> io::Result<()> {
    {
        // case 1
        let mut f = File::open("foo.txt")?;
        let mut buffer = [0; 10];

        // read up to 10 bytes;
        f.read_exact(&mut buffer)?;
        println!("{:?}", buffer);

        let mut buffer = vec![0; 10];
        // read the whole file
        f.read_to_end(&mut buffer)?;
        println!("{:?}", buffer);

        // read into a String, so that you don't need to do the conversion.
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        println!("{:?}", buffer);
    }

    {
        // case 2
        let mut b = "This string will be read".as_bytes();
        let mut buffer = [0; 10];

        // read up to 10 bytes
        b.read_exact(&mut buffer)?;
        println!("{:?}", buffer);
    }

    {
        // case 3
        let mut f = File::open("foo.txt")?;
        let mut buffer = [0; 10];

        // read exactly 10 bytes
        f.read_exact(&mut buffer)?;
        println!("{:?}", buffer);
    }

    {
        let f = File::open("foo.txt")?;

        for byte in f.bytes() {
            println!("{}", byte.unwrap());
        }
    }

    {
        let f1 = File::open("foo.txt")?;
        let f2 = File::open("bar.txt")?;

        let mut handle = f1.chain(f2);
        let mut buffer = String::new();

        // read the value into a String. Wecould use any Read method here,
        // this is just one example.
        handle.read_to_string(&mut buffer)?;
    }

    {
        let f = File::open("foo.txt")?;
        let mut buffer = [0; 5];

        // read at most five bytes
        let mut handle = f.take(5);

        handle.read_exact(&mut buffer)?;
    }

    // and more! See the other methods for more details.
    Ok(())
}