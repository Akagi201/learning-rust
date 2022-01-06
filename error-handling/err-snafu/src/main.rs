use snafu::{prelude::*, ErrorCompat, Whatever};

fn is_valid_id(id: u16) -> Result<(), Whatever> {
    if id < 10 {
        whatever!("id wrong");
    }
    Ok(())
}

#[allow(unused)]
fn read_config_file(path: &str) -> Result<String, Whatever> {
    std::fs::read_to_string(path)
        .with_whatever_context(|_| format!("Could not read file {}", path))
}

// #[derive(Debug, Snafu)]
// #[snafu(display("ID may not be less than 10, but it was {id}"))]
// struct InvalidIdError {
//     id: u16,
// }

// fn is_valid_id1(id: u16) -> Result<(), InvalidIdError> {
//     ensure!(id >= 10, InvalidIdSnafu { id });
//     Ok(())
// }

// #[derive(Debug, Snafu)]
// #[snafu(display("Could not read file {path}"))]
// struct ConfigFileError {
//     source: std::io::Error,
//     path: String,
// }

// #[allow(unused)]
// fn read_config_file1(path: &str) -> Result<String, ConfigFileError> {
//     std::fs::read_to_string(path).context(ConfigFileSnafu { path })
// }

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not read file {path}"))]
    ConfigFile {
        source: std::io::Error,
        path: String,
    },
}

#[allow(unused)]
fn read_config_file2(path: &str) -> Result<String, Error> {
    std::fs::read_to_string(path).context(ConfigFileSnafu { path })
}

#[derive(Debug, Snafu)]
enum MyError {
    #[snafu(display("ID may not be less than 10, but it was {id}"))]
    InvalidId { id: u16 },

    #[snafu(whatever, display("{message}"))]
    Whatever {
        message: String,
        #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
        source: Option<Box<dyn std::error::Error>>,
    },
}

fn is_valid_id2(id: u16) -> Result<(), MyError> {
    ensure!(id >= 10, InvalidIdSnafu { id });
    whatever!("Just kidding... this function always fails!");
    // Ok(())
}

fn main() {
    if let Err(e) = is_valid_id(1u16) {
        eprintln!("An error occurred: {}", e);
        if let Some(bt) = ErrorCompat::backtrace(&e) {
            eprintln!("{}", bt);
        }
    }

    // if let Err(e) = is_valid_id1(1u16) {
    //     eprintln!("An error occurred: {}", e);
    //     if let Some(bt) = ErrorCompat::backtrace(&e) {
    //         eprintln!("{}", bt);
    //     }
    // }

    if let Err(e) = is_valid_id2(1u16) {
        eprintln!("An error occurred: {}", e);
        if let Some(bt) = ErrorCompat::backtrace(&e) {
            eprintln!("{}", bt);
        }
    }
}
