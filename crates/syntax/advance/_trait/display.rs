use std::{
    error::Error,
    fmt::{self, Formatter},
    io,
};

// 自定义格式化输出
pub struct Point {
    x: i32,
    y: i32,
}

// 使用 Debug 的 展开格式化
impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

// Debug 的 {:?} 拍平格式化 及 {:#?} 展开格式化
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageCompress")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

// 为 Error 实现 Display
#[derive(Debug)]
enum ErrorDisplay {
    IoError(io::Error),
    EmptyUsername(String),
}

impl Error for ErrorDisplay {}

impl fmt::Display for ErrorDisplay {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO 错误: {e}"),
            Self::EmptyUsername(filename) => write!(f, "没有找到这个名字 {filename}"),
        }
    }
}
