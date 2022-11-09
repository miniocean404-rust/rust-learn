#[derive(Debug)]
enum Mode2 {
    V1,
}

enum Mode {
    V1,
    V2,
    Match(Mode2),
    V3,
    V4,
}

fn show_match(mode: Mode) -> u8 {
    match mode {
        Mode::V1 | Mode::V3 => 1,
        Mode::V2 => 2,
        Mode::Match(state) => {
            println!("{:#?}", state);
            25
        }
        // 本身需要列举枚举所有类型，但是不需要处理的可以 设置 _ => ()
        _ => 0,
    }
}

fn use_show_match() {
    let enum_value = Mode::Match(Mode2::V1);
    let value: u8 = show_match(enum_value);
}

// 范围匹配
fn range_match() {
    let x = 5;
    let y = 'c';

    match x {
        1..=6 => println!("1-5"),
        _ => println!("其他"),
    }

    match y {
        'a'..='j' => println!("字母"),
        _ => println!("其他"),
    }
}

// 结构体解构匹配
fn struct_match() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// 枚举解构匹配
fn enum_match() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

fn deconstruct_struct_enum_match() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}
