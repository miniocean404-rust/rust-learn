```rust
//! # 我的包
//! 我的包的描述

/// 文档注释：用于生成 HTML，可以使用 Markdown 语法，如果在 HTML 显示需要添加 pub
///
/// 常用 章节
///
/// # Example
/// - Panics 函数可能发生 panics 的场景
/// - Errors 函数返回 Rusult 描述可能错误的场景，以及导致错误的条件
/// - Safety 函数处于 unsafe 调用，应该解释不安全的原因，以及调用者确保使用的前提
///
/// 命令
/// cargo doc 生成文档(包含依赖项的文档)，会运行 rustdoc 工具，会将文档防止 target/doc 下
/// cargo doc --open 生成文档并打开
/// cargo test 可以运行文档注释用的代码块``` 包裹的 ```
///
/// # Example
/// ```
/// let a = 1;
/// let res = learn_base::use_doc();
/// ```
pub fn use_doc() {}
```
