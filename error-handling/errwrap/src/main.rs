#[derive(Debug)]
pub enum Error {
    IO(std::io::ErrorKind),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error.kind())
    }
}

fn do_read_file() -> Result<(), Error> {
    let data = std::fs::read("/tmp/foo")?;
    let data_str = std::str::from_utf8(&data).unwrap();

    println!("{}", data_str);
    Ok(())
}

fn main() -> Result<(), Error> {
    do_read_file()?;
    Ok(())
}
