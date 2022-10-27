use std::io;

fn main() {
    println!("Hello, world!");

    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("输入错误");

    print!("您的输入是 {}", input)
}
