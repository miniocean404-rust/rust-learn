#![allow(warnings)]

use std::fmt::Display;

// r 的生命周期长 ，将声明周期短的赋 r 后，短声明周期的内存已经释放所以是错误的
fn use_lift_position() {
    // 错误
    // let r;

    // {
    //     let x = 13;
    //     r = &x;
    // }

    // println!("{}", r);

    let x = 13; // x 生命周期为 13 行往下
    let r = &x; // y 生命周期为 14 行往下

    println!("{}", r);
}

// 标注：是描述多个引用的声明周期之间的关系，但是不影响生命周期
// 检查器不知道你参数的生命周期是哪个，有的参数的生命周期先销毁了，有的生命周期后销毁，所以使用 'a 标注是同一个生命周期，要保证一致
// 下面例子代表 x,y 的生命周期中 短的那个是 泛型的生命周期
// 从参数参会引用时，返回类型生命周期必须和参数的生命周期一致
fn use_lift_label<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 悬垂引用：函数中没有返回参数的引用，而是使用函数内部创建的引用，被调用时候这个引用直接被销毁，外部无法获取其值
// 如果想返回内部创建的值，直接返回值本身，将所有权转移给调用者
// fn use_lift_label<'a>() -> &'a str {
//     let str = String::from("1");
//     str.as_str()
// }

// 结构体标注：其内部值引用外部的字符串切片，这个切片的消失不能早于 Lift 这个结构体
struct Lift<'a> {
    day: &'a str,
}

impl<'a> Lift<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 这个生命周期因为有参数 &self 所以返回值的生命周期为 结构体的生命周期
    // 因为 结构体其内部值引用外部的字符串切片，这个切片的消失不能早于 Lift 这个结构体
    fn level2(&self, x: &str) -> i32 {
        3
    }
}

// 静态生命周期: 整个程序使用期间都存在
// 使用前考虑：是否需要在整个生命周期使用
fn use_lift_static() {
    // 直接存储在二进制程序里
    let str: &'static str = "静态生命周期";
}

// 例子：同时使用泛型和生命周期，生命周期也是泛型一种
fn use_lift_label1<'a, T>(x: &'a str, y: &'a str, z: T) -> &'a str
where
    T: Display,
{
    println!("{}", z);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
