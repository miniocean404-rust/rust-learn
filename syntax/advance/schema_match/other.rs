// #![allow(warnings)]

pub fn ignore_no_use_var() {
    let _x = 5;
}

struct Point {
    x: i32,
    y: i32,
}

// 解构 结构体 和 元祖
pub fn deconstruct_stuct_tmple() {
    let ((_feet, _inches), Point { x: _, y: _ }) = ((3, 10), Point { x: 3, y: -10 });
}

pub fn use_bind() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    // match 匹配范围时，绑定值到 新的变量里
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),

        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }

        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
