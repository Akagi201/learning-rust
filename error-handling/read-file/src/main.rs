fn main() {
    let path = "/tmp/dat"; // 文件路径
    match read_file(path) {
        // 判断方法结果
        Ok(file) => {
            println!("{}", file)
        } // OK 代表读取到文件内容，正确打印文件内容
        Err(e) => {
            println!("{} {}", path, e)
        } // Err 代表结果不存在，打印错误结果
    }
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    // Result 作为结果返回值
    std::fs::read_to_string(path) // 读取文件内容
}
