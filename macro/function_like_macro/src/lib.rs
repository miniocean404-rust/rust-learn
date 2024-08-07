use proc_macro::TokenStream;

/// 函数宏
/// 运行命令 `cargo r -p function_like_macro --example query`
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
