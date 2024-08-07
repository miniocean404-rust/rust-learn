# 命令

## 添加依赖

```
cargo add tokio --features full
```

## 打包

- `--manifest-path` 可选参数，指定 cargo.toml 文件

  ```shell
  cargo build --manifest-path ./Cargo.toml --release
  ```

## 构建 bin

```shell
cargo run --bin main
```

## 工作空间

1. workspace 根 cargo.toml 下不能设置 bin、package 启动项
2. -p 全称 --package 指定执行的包

   ```shell
   # 创建工作空间
   cargo new xx --lib|bin
   # 执行工作空间的 main.rs
   # 在指定 bin 时候指定 cargo run --package workspaceName --bin [workspace 下的包中的 bin] 这个指令中  --bin [bin 下文件] 的默认执行
   cargo run -p workspaceName
   ```

   ````
   # toolchain 命令

   ```shell
   # 格式化
   cargo fmt

   # 自动修复一部分
   cargo fix

   # lint 检查
   cargo clippy
   ````

## 执行 examples 文件

1. 根目录下创建 examples 文件夹，然后在 examples 文件夹下创建文件，执行命令
2. workspace 根目录下创建 examples 没有意义

```shell
cargo run --exmaple fileName.rs

# 执行 workspace 下的 examples 文件
cargo run -p workspaceName  --example query
```

# 语法

## 宏

文章：https://zhuanlan.zhihu.com/p/608978583
