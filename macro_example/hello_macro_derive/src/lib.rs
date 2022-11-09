extern crate proc_macro;

use proc_macro::TokenStream; // rust 自带
use quote::quote; // 将 syn 产生的数据结构重新转化为 Rust 代码
use syn; // 将字符串转化为可进一步操作的数据结构

// 用户使用 #[derive(HelloMacro)] 的时候 hello_macro_derive 函数就会被自动调用
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // syn 解析后的结构
    // DeriveInput {
    //     // --snip--

    //     ident: Ident {
    //         ident: "Pancakes",
    //         span: #0 bytes(95..103)
    //     },
    //     data: Struct(
    //         DataStruct {
    //             struct_token: Struct,
    //             fields: Unit,
    //             semi_token: Some(
    //                 Semi
    //             )
    //         }
    //     )
    // }

    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // quote!宏 定义需要返回的 Rust 代码
    let gen = quote! {
        // #name 会替换为 name 的值
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! 将表达式转化为字面量字符串的值
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    // gen.into 将代码 转化为 TokenStream
    gen.into()
}
