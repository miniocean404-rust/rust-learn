mod builder;
mod raw_builder;

use crate::raw_builder::j2::BuilderContext;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident, ItemFn, Type};

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
    let _input = parse_macro_input!(input as DeriveInput);

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

#[proc_macro_derive(Optional, attributes(optional))]
pub fn optional(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let Data::Struct(data_struct) = input.data else {
        // 不接受除了结构体之外的类型
        return syn::Error::new(ident.span(), "optional can only be applied to structs")
            .into_compile_error()
            .into();
    };

    let optional_struct_name = &format!("Optional{}", ident); // OptionalPicea
    let optional_struct_ident = Ident::new(optional_struct_name, ident.span());

    let fields: Vec<_> = data_struct
        .fields
        .iter()
        .map(|field| {
            // 对每个字段进行映射
            let mut ident = field.ident.clone();
            let ty = &field.ty;

            let mut is_skip = false;

            let attr = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("optional"));

            if let Some(attr) = attr {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        is_skip = true;
                    } else if meta.path.is_ident("rename") {
                        let renamed_ident = meta.value()?.parse::<syn::Ident>()?;
                        ident = Some(renamed_ident);
                    }
                    Ok(())
                });
            }

            if is_skip || is_option(ty) {
                return quote::quote!(#ident: #ty);
            }

            quote::quote!(#ident: Option<#ty>)
        })
        .collect();

    quote::quote! {
        struct #optional_struct_ident {
            #(#fields,)*
        }
    }
    .into()
}

fn is_option(ty: &Type) -> bool {
    let Type::Path(path) = ty else {
        return false;
    };
    let path = &path.path;
    path.segments
        .last()
        .map(|segment| segment.ident == "Option")
        .unwrap_or(false)
}
