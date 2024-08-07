# macro_rules! 声明宏

## 简介

声明宏: 它是一种基于模式匹配的文本替换宏，类似于 C 语言中的宏定义。声明宏在编译期展开，用匹配的代码片段替换宏调用处的代码

## 常见的 Rust 宏选择器

下面是一些常见的 Rust 宏选择器(Designator)：

- item：条目，例如函数、结构、模块等
- block：代码块
- stmt：语句
- pat：模式
- expr：表达式
- ty：类型
- ident：标识符
- path：路径，例如 foo、 ::std::mem::replace, transmute::<\_, int>, ...
- meta：元信息条目，例如 #[...]和 #![rust macro...] 属性
- tt：词条树

## 示例

### 简单使用

在匹配器(Matcher)中，$name 部分定义了变量名，匹配结果将绑定到该变量以便应用到转码器(Transcriber)中。

在这个示例中，表示我们将 Rust 宏的匹配结果存入变量 $name。冒号后面的部分被称为选择器(Designator)，用于声明我们要匹配的类型。

例如在这个示例中，我们使用的是表达式选择器，也就是 expr，这告诉 Rust：匹配一个表达式，然后存入 $name 变量。

```rust
#[macro_export]
macro_rules! learn_macro {
    ($name:expr) => {
        println!("一个参数：{:?} = {:?}", stringify!($expr), $key1);
    };

    ($key1:expr,$key2:expr) => {
        println!(
            "两个参数：{:?} = {:?} {:?}",
            stringify!($expr),
            $key1,
            $key2
        );
    };
}
```

### 匹配多个

在 Rust 宏中，重复模式允许你匹配任意数量的参数。重复模式的语法是 $()，我们只需要把希望重复的模式写在$(...)这部分。

并且可以指定分隔符(,)在这里也就是逗号重复的次数(\*|+)，表示重复匹配$()中的模式

我们使用模式 `$key:expr => $value:expr 来分别捕捉` $key 和 $value 表达式，分隔符为 =>。不过现在只能匹配一个键 / 值对，但是哈希表通常都是多个键值对的
在这个宏中，$($x:expr),* 匹配任意数量的表达式，每个表达式之间用逗号分隔。$() 内的代码会对每个匹配的表达式重复执行，包括执行代码的语句一样也是。

```rust
#[macro_export]
macro_rules! map {
    ($( $key:expr => $value:expr ),*) => {
        let mut hm = HashMap::new();
        // $()*，它的意思是其中的代码会重复展开为多次一样的代码，不一样的参数
        $(hm.insert( $key, $value);)*
    };

    ($( $key:expr => $value:expr ),*) => {
        $(println!("{},{}", $key, $value);)*
    };
}
```
