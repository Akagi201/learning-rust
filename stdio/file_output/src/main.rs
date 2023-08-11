// 输出文本
static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::{fs::File, io::prelude::*, path::Path};

fn main() {
    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();

    // 用只写模式打开一个文件，并返回 `io::Result<File>` 类型
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, &why),
        Ok(file) => file,
    };

    // 写入 `LOREM_IPSUM` 字符串到文件中，并返回 `io::Result<()>` 类型
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display, &why)
        }
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
