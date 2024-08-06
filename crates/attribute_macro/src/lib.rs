use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// # 示例
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
pub fn my_attribute(attrs: TokenStream, item: TokenStream) -> TokenStream {
    // let args = parse_macro_input!(attrs as syn::Attribute);
    let name = attrs.to_string();
    println!("传递函数、结构体参数名字:{}", name);
    println!("添加宏的函数字符串:{}", item);

    let mut result = item.to_string();

    // r# 不用添加转义符号
    result.push_str(&format!(
        r#"
            fn {}() {{
                println!("这是一个由属性宏生成的自定义函数!");
            }}
        "#,
        name
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
///
/// ```
/// #[log_func_info]
/// fn example_function() {
///     println!("这是函数体内的代码");
/// }
///
/// fn main() {
///     example_function();
/// }
///
/// ```
///
/// 结果：函数开始 example_function
/// 这是函数体内的代码
/// 函数结束 example_function
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
