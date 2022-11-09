// 宏（Macro）指的是 Rust 中一系列的功能：
//      使用 macro_rules! 的 声明（Declarative）宏
//      三种 过程（Procedural）宏：
//          自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
//          类属性（Attribute-like）宏定义可用于任意项的自定义属性
//          类函数宏看起来像函数不过作用于作为参数传递的 token

// 宏和函数的区别
// 从根本上来说，宏是一种为写其他代码而写代码的方式，即所谓的 元编程（metaprogramming）。在附录 C 中会探讨 derive 属性，
// 其生成各种 trait 的实现。我们也在本书中使用过 println! 宏和 vec! 宏。所有的这些宏以 展开 的方式来生成比你所手写出的更多的代码。
// 元编程对于减少大量编写和维护的代码是非常有用的，它也扮演了函数扮演的角色。但宏有一些函数所没有的附加能力。

// 一个函数签名必须声明函数参数个数和类型。
// 相比之下，宏能够接收不同数量的参数：用一个参数调用 println!("hello") 或用两个参数调用 println!("hello {}", name) 。
// 而且，宏可以在编译器翻译代码前展开，例如，宏可以在一个给定类型上实现 trait 。
// 而函数则不行，因为函数是在运行时被调用，同时 trait 需要在编译时实现。

// 实现宏不如实现函数的一面是宏定义要比函数定义更复杂，
// 因为你正在编写生成 Rust 代码的 Rust 代码。由于这样的间接性，宏定义通常要比函数定义更难阅读、理解以及维护。

// 宏和函数的最后一个重要的区别是：在一个文件里调用宏 之前 必须定义它，或将其引入作用域，而函数则可以在任何地方定义和调用。

// 声明宏的例子：
// let v: Vec<u32> = vec![1, 2, 3]; vec! 宏的实现
#[macro_export]
// macro_rules! 声明一个 vec 宏
macro_rules! vec {
    // ( $( $x:expr ),* ) 是一种匹配
    // $x:expr 表示 rust 表达式
    // (),* ,代表可能会出现的 , *代表匹配多次
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// 过程宏
// 这种形式更像函数（某种形式的过程)一些
//      接收并操作输入的RUst代码
//      生成另外一些RUst代码作为结果
// 三种过程宏：
//      自定义派生(derive)
//      属性宏
//          属性宏与自定义 derive 宏类似
//          允许创建新的属性但不是为 derive 属性生成代码
//          属性宏更加灵活:
//          derive 能用于struct 和 enum
//          属性宏可以用于任意条目,例如函数
//              #[route(GET, "/")]
//              fn index() {}
//
//              #[proc_macro_attribute]
//              attr 为 GET, "/" 参数 item 为 index() {} 函数体
//              pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
//      函数宏
//              let sql = sql!(SELECT * FROM posts WHERE id=1);
//
//              #[proc_macro]
//              pub fn sql(input: TokenStream) -> TokenStream {
// 创建过程宏时：
//      宏定义必须单独放在它们自己的包中，并使用特殊的包类型
