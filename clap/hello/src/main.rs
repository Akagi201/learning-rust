extern crate clap;
use clap::App;

fn main() {
    App::new("hello")
        .version("0.1.0")
        .about("Does great things!")
        .author("Akagi201")
        .get_matches();
}
