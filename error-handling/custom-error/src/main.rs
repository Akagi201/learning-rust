use std::error::Error;

///自定义类型 Error，实现 std::fmt::Debug 的 trait
#[derive(Debug)]
struct CustomError {
    err: ChildError,
}

///实现 Display 的 trait，并实现 fmt 方法
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomError is here!")
    }
}

///实现 Error 的 trait，因为有子 Error:ChildError，需要覆盖 source() 方法，返回 Some(err)
impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.err)
    }
}


///子类型 Error，实现 std::fmt::Debug 的 trait
#[derive(Debug)]
struct ChildError;

///实现 Display 的 trait，并实现 fmt 方法
impl std::fmt::Display for ChildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChildError is here!")
    }
}

///实现 Error 的 trait，因为没有子 Error，不需要覆盖 source() 方法
impl std::error::Error for ChildError {}

///构建一个 Result 的结果，返回自定义的 error:CustomError
fn get_super_error() -> Result<(), CustomError> {
    Err(CustomError { err: ChildError })
}

fn main() {
    match get_super_error() {
        Err(e) => {
            println!("Error: {}", e);
            println!("Caused by: {}", e.source().unwrap());
        }
        _ => println!("No error"),
    }
}