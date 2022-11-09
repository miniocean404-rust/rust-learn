// 解构 结构体 和 元祖
fn deconstruct_stuct_tmple() {
    // let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

fn ignore_no_use_var() {
    let _x = 5;
    let y = 10;
}

fn ignore_some_val() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
        (first, .., fifth) => {
            println!("Some numbers: {},  {}", first, fifth)
        }
        (.., second) => {
            println!("Some numbers: {}", second)
        }
    };
}

fn ignore_other_val() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

// match 分支模式之后的额外 if 条件
fn match_guard() {
    let num = Some(4);
    let x = 4;
    let y = false;

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn use_bind() {
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
