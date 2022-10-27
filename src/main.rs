use std::cmp::Ordering;
use std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("输入错误");

    println!("您的输入是 {}", input);

    let comper: String = String::new();

    match comper.cmp(&input) {
        Ordering::Less => println!("小于"),
        Ordering::Greater => println!("大于"),
        Ordering::Equal => println!("差不多"),
    }
}
