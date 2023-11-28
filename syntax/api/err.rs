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
