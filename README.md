# 命令

## 打包

```shell
cargo build --release
```
## 构建 bin

```shell
cargo run --bin main
```

## 工作空间
```shell
# 创建工作空间
cargo new xx --lib
# 执行工作空间的 main.rs 
cargo run -p workspaceName
```

## toolchain 命令

```shell
# 格式化
cargo fmt

# 自动修复一部分
cargo fix

# lint 检查
cargo clippy
```

## 执行 examples 文件
根目录下创建 examples 文件夹，然后在 examples 文件夹下创建文件，执行命令
```shell
cargo run --exmaple fileName.rs
```

# 语法

## 宏
文章：https://zhuanlan.zhihu.com/p/608978583
