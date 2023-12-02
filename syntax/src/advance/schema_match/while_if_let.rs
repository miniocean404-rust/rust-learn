#![allow(warnings)]
// 能匹配任何传递的可能值的模式被称为是 不可反驳的（irrefutable）。一个例子就是 let x = 5; 语句中的 x，因为 x 可以匹配任何值所以不可能会失败。
// 对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable）。一个这样的例子便是 if let Some(x) = a_value 表达式中的 Some(x)；如果变量 a_value 中的值是 None 而不是 Some，那么 Some(x) 模式不能匹配

// 函数参数、 let 语句和 for 循环只能接受不可反驳的模式，因为通过不匹配的值程序无法进行有意义的工作。
// if let 和 while let 表达式被限制为只能接受可反驳的模式，因为根据定义他们意在处理可能的失败：条件表达式的功能就是根据成功或失败执行不同的操作。

// match匹配分支必须使用可反驳模式，除了最后一个分支需要使用能匹配任何剩余值的不可反驳模式。
use std::vec;

// match 的语法糖
fn show_if_let() {
    let value = Some(0u8);

    if let Some(2) = value {
        println!("two")
    } else {
        println!("other")
    }
}

pub fn show_while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

pub fn for_iter() {
    let v = ['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

pub fn use_deconstruct() {
    let point = (3, 5);
    deconstruct(&point);
}

pub fn deconstruct(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

// 带有两个字段 x 和 y 的结构体 Point，可以通过带有模式的 let 语句将其分解：
struct Point {
    x: i32,
    y: i32,
}

fn deconstruct_struct() {
    let p = Point { x: 0, y: 7 };

    let Point { x: _a, y: _b } = p;
}
