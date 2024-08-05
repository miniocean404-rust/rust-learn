#![allow(dead_code)]

// 1. rust 没有 null 使用 Option 中的 None 代替，
// 2. Option<i32> 不能当做 i32, 就是不能直接和 i32 进行数据加减的相关性操作意思
fn use_option() {
    let x: Option<i32> = Some(1);
    let y: Option<i32> = None;
}

fn match_option(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
