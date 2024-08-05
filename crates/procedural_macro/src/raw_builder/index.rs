use proc_macro::{Ident, TokenStream, TokenTree};
use std::collections::VecDeque;

use super::j2::Fd;

/// 把 TokenStream 分出 struct 的名字，和包含 fields 的 TokenStream
pub fn split(input: TokenStream) -> (Ident, TokenStream) {
    let mut input = input.into_iter().collect::<VecDeque<_>>();
    // 一直往后找，找到 struct 停下来
    while let Some(item) = input.pop_front() {
        if let TokenTree::Ident(v) = item {
            if v.to_string() == "struct" {
                break;
            }
        }
    }

    // struct 后面，应该是 struct name
    let ident;
    if let Some(TokenTree::Ident(v)) = input.pop_front() {
        ident = v;
    } else {
        panic!("Didn't find struct name");
    }

    // struct 后面可能还有若干 TokenTree，我们不管，一路找到第一个 Group
    let mut group = None;
    for item in input {
        if let TokenTree::Group(g) = item {
            group = Some(g);
            break;
        }
    }

    (ident, group.expect("Didn't find field group").stream())
}

/// 核心方法，从包含 fields 的 TokenStream 中切出来一个个 Fd，例如把一个 a=1,b=2 的字符串切成 [[a, 1], [b, 2]]
pub fn get_struct_fields(input: TokenStream) -> Vec<Fd> {
    let input = input.into_iter().collect::<Vec<TokenTree>>();

    input
        .split(|v| {
            match v {
                // 先用 ',' 切出来一个个包含 field 所有信息的 &[TokenTree]
                TokenTree::Punct(p) => p.as_char() == ',',
                _ => false,
            }
        })
        .map(|tokens| {
            tokens
                .split(|v| match v {
                    // 再用 ':' 把 &[TokenTree] 切成 [&[TokenTree], &[TokenTree]]
                    // 它们分别对应名字和类型
                    TokenTree::Punct(p) => p.as_char() == ':',
                    _ => false,
                })
                .collect::<Vec<_>>()
        })
        // 正常情况下，应该得到 [&[TokenTree], &[TokenTree]]，对于切出来长度不为 2 的统统过滤掉
        .filter(|tokens| tokens.len() == 2)
        // 使用 Fd::new 创建出每个 Fd
        .map(|tokens| Fd::new(tokens[0], tokens[1]))
        .collect()
}
