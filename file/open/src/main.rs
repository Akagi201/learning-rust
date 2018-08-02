use std::fs::File;
 
fn main() {
   let _f = File::open("file.txt");
 
   let _f = match _f {
      Ok(file) => file,
      Err(why) => panic!("Error opening the file {:?}", why),
   };
}
 
// thread 'main' panicked at 'Error opening the file Error { repr: Os 
// { code: 2, message: "No such file or directory" } }', recover.rs:8:23
// note: 使用 RUST_BACKTRACE=1 运行，以便实现反向跟踪。