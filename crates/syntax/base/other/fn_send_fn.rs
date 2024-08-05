#![allow(dead_code)]

fn add_one(x: i32) -> i32 {
    x + 1
}

// 不同于闭包，fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数
// 函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），
// 所以总是可以在调用期望闭包的函数时传递函数指针作为参数。倾向于编写使用泛型和闭包 trait 的函数，这样它就能接受函数或闭包作为参数

// 一个只期望接受 fn 而不接受闭包的情况的例子是: 与不存在闭包的外部代码交互时：C 语言的函数可以接受函数作为参数，但 C 语言没有闭包
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// 可以同时传递函数和闭包
// 接收的参数是函数指针
pub fn use_send_fn() {
    let answer = do_twice(add_one, 5);

    println!("回答是: {}", answer);
}

// 返回闭包
// 直接返回的话，Rust 并不知道需要多少空间来储存闭包。
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
