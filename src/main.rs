use std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("输入错误");

    println!("您的输入是 {}", input)
}
