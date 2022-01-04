use std::path::Path;

fn file_name(file_path: &str) -> Option<&str> {
    let path = Path::new(file_path);
    path.file_name().unwrap().to_str()
}

fn file_path_ext(file_path: &str) -> Option<&str> {
    file_name(file_path).and_then(extension)
}

fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i+1..]),
    }
}

// map 是标准库中的方法
// fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
//     match option {
//         None => None,
//         Some(value) => Some(f(value)),
//     }
// }

// 使用 map 去掉 match
fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}

fn main() {
    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("No file extension found."),
        Some(i) => println!("File extension: {}", &file_name[i+1..]),
    }

    match extension_explicit("foo.rs") {
        None => println!("no extension"),
        Some(ext) => assert_eq!(ext, "rs"),
    }

    match extension("foo.rs") {
        None => println!("no extension"),
        Some(ext) => assert_eq!(ext, "rs"),
    }

    assert_eq!(extension("foo.rs").unwrap_or("rs"), "rs");
    assert_eq!(extension("foo").unwrap_or("rs"), "rs");
}