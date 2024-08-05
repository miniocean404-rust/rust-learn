// https://juejin.cn/post/7264503343997304886#heading-5
// https://www.jianshu.com/p/fc0ed4f4d661

// 过程宏：是一种更为高级的宏，它通过编写 Rust 代码来处理输入的代码，并在编译期间生成新的代码。
//         过程宏主要用于属性宏（Attribute Macros）、
//         类函数宏（Function-Like Macros）
//         派生宏（Derive Macros）等场景。

// 属性宏在Rust中具有以下几个特点：
// 1. 代码定制化处理：属性宏允许开发者在代码上方添加自定义的属性，并根据属性的输入对代码进行定制化处理。这使得开发者可以根据需要修改代码的结构和行为。
// 2. 编译期间执行：属性宏在编译期间执行，而不是运行时执行。这意味着宏生成的代码在编译时就已经确定，不会增加运行时的性能开销。
// 3. 代码安全性：属性宏生成的代码必须是合法的Rust代码，它们受到Rust编译器的类型检查和安全检查。这保证了宏生成的代码不会引入潜在的编译错误和安全漏洞。

// 属性宏的局限性
// 虽然属性宏在Rust中非常强大，但它也有一些局限性需要注意：
// 1. 仅适用于特定项：属性宏只能应用于函数、结构体、枚举等特定的项，而不能应用于表达式等其他类型的代码。
// 2. 无法修改输入项：属性宏只能生成新的代码，而不能修改输入项的内容。例如，无法在函数内部添加新的语句或修改函数的签名。
// 3. 不支持模式匹配：与声明宏不同，属性宏不能进行模式匹配，只能对整个输入项进行处理。
mod builder;
mod raw_builder;

use crate::raw_builder::j2::BuilderContext;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

/// # Examples
/// ```
/// #[my_attribute(hello)]
/// fn use_attribute_derive() {
///     println!("函数本身的调用")
/// }
/// fn main() {
///     hello();
/// }
///
/// ```
/// 还可以添加参数，在 attr 中可以获取
/// #[route(hello,"/")]
#[proc_macro_attribute]
pub fn my_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function_name = attr.to_string();
    println!("传递函数、结构体参数名字:{}", function_name);
    println!("添加宏的函数字符串:{}", item);

    let mut result = item.to_string();

    // r# 不用添加转义符号
    result.push_str(&format!(
        r#"
            fn {}() {{
                println!("这是一个由属性宏生成的自定义函数!");
            }}
        "#,
        function_name
    ));

    result.parse().unwrap()
}

/// # Examples
/// ```
/// #[my_struct(Point)]
/// fn use_attribute_derive() {
///     println!("函数本身的调用")
/// }
///
/// fn main() {
///     let point = Point { data: 10 };
///     println!("Data: {}", point.data); // 输出：Data: 10
/// }
/// ```
#[proc_macro_attribute]
pub fn my_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_name = attr.to_string();
    let mut result = item.to_string();

    result.push_str(&format!(
        r#"
            struct {} {{
                data: i32
            }}
        "#,
        struct_name
    ));

    result.parse().unwrap()
}

/// 条件编译
/// cfg 判断 feature 的值是否开启需要：
/// 1. 在库文件 Cargo.toml 中添加
/// ```
/// [features]
/// my_feature = []
/// ```
/// 2. 在调用库的项目中的依赖中添加
/// ```
/// process-derive = { path = "./process-derive", features=["my_feature"] }
/// ```
///
/// #[my_feature]
/// fn test() {
///     my_function();
/// }
/// ```
#[proc_macro_attribute]
pub fn my_feature(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut result = item.to_string();

    #[cfg(feature = "my_feature")]
    result.push_str(
        r#"
                    fn my_function() {{ println!("my_feature 开启了"); }}
                "#,
    );

    #[cfg(not(feature = "my_feature"))]
    result.push_str(
        r#"
                    fn my_function() {{ println!("my_feature 关闭了"); }}
        "#,
    );

    result.parse().unwrap()
}

#[proc_macro_attribute]
pub fn log_func_info(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);
    let func_name = &func.sig.ident;
    let func_block = &func.block;

    let output = quote! {
        {
            println!("函数开始", stringify!(#func_name));
            let __log_result = { #func_block };
            println!("函数结束", stringify!(#func_name));
            __log_result
        }
    };

    func.block = syn::parse2(output).unwrap();
    quote! { #func }.into()
}

// 函数宏
#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    // 这里面 TokenStream 是一个 Iterator，里面包含一系列的 TokenTree：
    // TokenTree：
    // 后面三个分别是
    //  1. Ident（标识符）、
    //  2. Punct（标点符号）
    //  3. Literal（字面量）。
    // 这里的 Group（组）是因为如果代码中包含括号（比如{} [] <> () ），那么内部的内容会被分析成一个 Group（组）。
    // 可以把例子中对 query! 的调用改成以下
    // query!(SELECT * FROM users u JOIN (SELECT * from profiles p) WHERE u.id = p.id and u.age > 10);
    // 这个样子，再运行一下，此时 TokenStream 就包含了 Group：

    println!("{:#?}", input);
    r#"
    fn hello() {
        println!("Hello world!");
    }
    "#
    .parse()
    .unwrap()
}

// 派生宏是一种特殊的宏，它允许开发者为自定义的数据类型自动实现trait

// 派生宏在Rust中具有以下几个特点：
// 1. 自动实现trait：派生宏允许开发者为自定义的数据类型自动实现trait，无需手动编写trait的实现代码。这样可以大大减少重复的代码，提高代码的可读性和可维护性。
// 2. 编译期间执行：派生宏的逻辑在编译期间执行，而不是运行时执行。这意味着trait的实现代码在编译时就已经确定，不会增加运行时的性能开销。
// 3. 代码安全性：派生宏生成的trait实现代码必须是合法的Rust代码，它们受到Rust编译器的类型检查和安全检查。这保证了派生宏生成的trait实现不会引入潜在的编译错误和安全漏洞。

// 派生宏的局限性
// 1. 虽然派生宏在Rust中非常强大，但它也有一些局限性需要注意：
// 2. trait的限制：派生宏只能自动实现由Rust标准库或第三方库定义的trait，无法自动实现用户自定义的trait。
// 3. 复杂数据结构的支持：对于一些复杂的数据结构，特别是包含泛型参数或嵌套类型的数据结构，派生宏可能无法处理。
// 4. 代码生成的安全性：由于派生宏是在编译期间执行，生成的代码必须是合法的Rust代码。如果宏的处理逻辑出现错误，可能会导致编译错误或不符合预期的代码生成。

#[proc_macro_derive(YourTrait, attributes(attr1, attr2))]
pub fn your_derive_macro(input: proc_macro::TokenStream) -> TokenStream {
    // struct Person { name : String, age : u32, }
    let struct_frag = input.to_string();
    let input = parse_macro_input!(input as DeriveInput);

    // 解析属性参数
    let _attr1 = if struct_frag.contains("attr1") {
        r#"
            impl YourTrait1 for YourType {{
                // 根据 attr1 生成的trait实现
            }}
        "#
    } else {
        ""
    };

    let _attr2 = if struct_frag.contains("attr2") {
        r#"
        impl YourTrait1 for YourType {{
            // 根据 attr2 生成的trait实现
        }}
    "#
    } else {
        ""
    };

    let result = r#"
            use core::fmt::Debug;
            impl Debug for Person {
                fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {todo!()}
                // 自动实现 trait 的代码
            }
        "#
    .to_string();

    println!("{}", result);
    result.parse().unwrap()
}

#[proc_macro_derive(RawBuilder)]
pub fn derive_raw_builder(input: TokenStream) -> TokenStream {
    println!("input的值是 {:#?}", input);

    BuilderContext::render(input).unwrap().parse().unwrap()
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // println!("input的值是 {:#?}", input);

    builder::index::BuilderContext::from(input).render().into()

    // TokenStream::default()
}
