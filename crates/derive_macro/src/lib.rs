use proc_macro::{TokenStream};
use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput, Ident, Type};

use crate::j2_template_builder::j2::BuilderContext;

mod syn_builder;
mod j2_template_builder;

/// # 参数解释
/// input: 派生宏调用的输入
/// # 示例：
/// 在 examples 中的 person_macro_demo
/// 运行 `cargo r -p derive_macro --example add_debug_macro_demo`
#[proc_macro_derive(PersonMacro, attributes(attr1, attr2))]
pub fn add_debug_macro(input: TokenStream) -> TokenStream {
    // 返回应用的结构体的字符串
    let struct_frag = input.to_string();
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident; // 结构体的名字, 在这里就是 "Person"

    // 判断字符串中是否包含 宏 的某个属性
    let attr1 = if struct_frag.contains("attr1") {
        format!(r#"
            trait PersonGetName {{
                fn get_name(&self) -> &String;
            }}

            impl PersonGetName for {} {{
                 fn get_name(&self) -> &String {{
                    &self.name
                 }}
            }}
        "#, struct_name)
    } else {
        "".to_string()
    };

    let attr2 = if struct_frag.contains("attr1") {
        format!(r#"
            trait PersonGetAge {{
                fn get_age(&self) -> &u32;
            }}

            impl PersonGetAge for {} {{
                 fn get_age(&self) -> &u32 {{
                    &self.age
                 }}
            }}
        "#, struct_name)
    } else {
        "".to_string()
    };

    let debug = r#"
            use std::fmt::{Debug, Formatter};

            // 自动实现 trait 的代码
            impl Debug for Person {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                   write!(f, "Person {{ name: {}, age: {} }}", self.name, self.age)
                }
            }
    "#;


    let result = format!("{}{}{}", attr1, attr2, debug);
    println!("{}", &result);
    result.parse().unwrap()
}

/// # 方法解释：
/// 使用模板引擎构建宏
#[proc_macro_derive(TemplateEngine)]
pub fn use_template_engine_sonstruct_macro(input: TokenStream) -> TokenStream {
    // 方式 1:
    println!("input的值是 {:#?}", input);
    println!("TokenStream 默认值是 {:#?}", TokenStream::default());
    BuilderContext::render(input).unwrap().parse().unwrap()

    // 方式 2:
    // let input = parse_macro_input!(input as DeriveInput);
    // println!("input的值是 {:#?}", input);
    // builder::index::BuilderContext::from(input).render().into()
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // println!("input的值是 {:#?}", input);

    syn_builder::builder::BuilderContext::from(input).render().into()

    // TokenStream::default()
}


// https://juejin.cn/post/7344567594086400015?share_token=648fd1f7-8da3-4b45-ad2f-34af240a2411
#[proc_macro_derive(Optional, attributes(optional))]
pub fn optional(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let Data::Struct(data_struct) = input.data else {
        // 不接受除了结构体之外的类型
        return syn::Error::new(ident.span(), "Optional 只能应用于结构")
            .into_compile_error()
            .into();
    };

    // 生成的结构体名字
    let optional_struct_name = &format!("Optional{}", ident);
    // 生成的结构体的标识符
    let optional_struct_ident = Ident::new(optional_struct_name, ident.span());

    let fields: Vec<_> = data_struct
        .fields
        .iter()
        .map(|field| {
            // 对每个字段进行映射(id，subs，descriptions)
            let mut ident = field.ident.clone();
            // dbg!(ident.to_token_stream().to_string());

            // 获取字段的类型
            let ty = &field.ty;

            let mut is_skip = false;

            // 对字段上的宏属性进行过滤
            let attr = field
                .attrs
                .iter()
                .find(|attr| {
                    // println!("attr.path() -> 获取属性的属性名 {:?}", attr.path().to_token_stream().to_string());
                    attr.path().is_ident("optional")
                });

            if let Some(attr) = attr {
                let _ = attr.parse_nested_meta(|meta| {

                    // #[optional(rename = value)] 中的 rename
                    // println!("meta.path -> 获取属性的属性的参数 {:?}", meta.path.to_token_stream().to_string());
                    if meta.path.is_ident("skip") {
                        is_skip = true;
                    } else if meta.path.is_ident("rename") {
                        let value = meta.value()?;
                        // 获取属性的属性的参数的值并转为标识符 #[optional(rename = value)] 中的 value
                        let renamed_ident = value.parse::<syn::Ident>()?;

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

    // dbg!("{:?}", quote::quote! {
    //     struct #optional_struct_ident {
    //         #(#fields,)*
    //     }
    // });

    let token = quote::quote! {
        struct #optional_struct_ident {
            #(#fields,)*
        }
    };

    // let show: TokenStream = token.clone().into();
    // dbg!(show.to_string());
    token.into()
}

// 判断是否是 Option 类型
fn is_option(ty: &Type) -> bool {
    let Type::Path(path) = ty else {
        return false;
    };

    // 获取类型
    let path = &path.path;

    path.segments
        .last()
        .map(|segment| segment.ident == "Option")
        .unwrap_or(false)
}
