#![allow(warnings)]
use std::io;
use thiserror::Error;

// 模拟从其他库中导入的错误类型
#[derive(Debug, Clone)]
pub struct OtherLibError;

/// 搭配 anyhow 使用
/// ```
/// Err(anyhow!(CustromError::WebdriverConnectionError))?;
/// bail!(CustromError::WebdriverConnectionError); // bail! 是 Err(anyhow!()) 的简写
///
/// Err::<(), CustromError>(CustromError::WebdriverConnectionError)?;
/// Err(CustromError::IoError(io::Error::new(io::ErrorKind::Other, "发生原因")))?; // #[from] 使用
/// ```
#[derive(Error, Debug)]
pub(crate) enum CustromError {
    #[error("无法成功连接")]
    ConnectionError,

    #[error("无法成功连接redis {0}")]
    RedisError(String),

    #[error("环境变量:{key:?}错误")]
    InvalidConfig { key: String },

    // IoError from 后的类型表示由什么类型转化来变成的当前的错误类型 io::Error -> CustromError::IoError
    // 如果 read_to_string(file_path).map_err(MyError::from) 返回错误，我们使用 MyError::from 将它转换为 MyError::IoError
    #[error("发生 io 错误")]
    IoError(#[from] io::Error),

    // #[error(transparent)]属性意味着该错误只是作为其他错误的容器，它的错误消息将直接从其“源”错误中继承。
    /// ```
    /// let err = MyError::from(anyhow!("Missing attribute: {}", "field1"));
    /// println!("{:?}", err);
    /// ```
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),

    // #[source]
    // 可以使用 #[source] 属性，或者将字段命名为 source，可为自定义错误实现 source 方法，返回底层的错误类型：
    /// ```
    /// let err = MyError::IO {
    ///     err: io::Error::from(io::ErrorKind::TimedOut),
    /// };
    /// println!("{:?}", err.source());
    ///
    /// let err = CustromError::from(io::Error::from(io::ErrorKind::TimedOut));
    /// println!("{:?}", err.source());
    /// ```
    ///
    #[error("some io error happened, {:?}", .source)]
    IO1 { source: io::Error },

    // 或者使用 #[source] 属性标记非 source 的字段，例如：这里是 err 字段
    #[error("some io error happened, {:?}", .err)]
    IO2 {
        #[source]
        err: io::Error,
    },
    // 回溯
    // #[error("some io error happened, {:?}", .err)]
    // IO3 {
    //     #[source]
    //     err: io::Error,
    //     #[backtrace]
    //     backtrace: Backtrace,
    // },
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct MyError1 {
    #[from]
    source: anyhow::Error,
}
