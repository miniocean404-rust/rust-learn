// main 函数返回 Result , 在 main 中使用 ? 处理 Result
// use learn_base::api::use_err;
// use std::error::Error;
// fn main() -> Result<(), Box<dyn Error>> {
//     use_err::use_err()?;
//     Ok(())
// }

use std::{
    fs::File,
    io::{self, Read},
};

use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub fn use_err() -> Result<String, io::Error> {
    let mut s: String = String::new();
    File::open("./hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// fn use_unwarp_or_else() {
//     File::open("./hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("./hello.txt").unwrap_or_else(|error| panic!("错误的创建文件 {:?}", error))
//         } else {
//             panic!("错误的打开文件 {:?}", error);
//         }
//     });
// }

//! 将错误转化：
// 此处的 From::from 调用意味着我们尝试将错误类型转换为 函数返回的类型。这使得很容易将错误封装到 更高级别的错误。
/// ```rs
/// match expression {
///     Ok(value) => value,
///     Err(err)  => return Err(From::from(err)),
/// }
/// ```
#[derive(Debug)]
enum ReadUsernameError {
    IoError(io::Error),
    EmptyUsername(String),
}

impl Error for ReadUsernameError {}

impl Display for ReadUsernameError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO 错误: {e}"),
            Self::EmptyUsername(filename) => write!(f, "没有找到这个名字 {filename}"),
        }
    }
}

impl From<io::Error> for ReadUsernameError {
    fn from(err: io::Error) -> ReadUsernameError {
        ReadUsernameError::IoError(err)
    }
}

fn read_username(path: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::with_capacity(100);
    File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(String::from(path)));
    }
    Ok(username)
}

fn main() {
    //fs::write("config.dat", "").unwrap();
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
}
