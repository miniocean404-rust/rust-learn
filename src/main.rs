use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let radom: i32 = rand::thread_rng().gen_range(0..=100);
    println!("神秘数字 {}", radom);

    println!("{}", 2);
    loop {
        let mut input: String = String::new();

        io::stdin().read_line(&mut input).expect("输入错误");

        println!("input {}", input);

        let input: i32 = match input.trim().parse() {
            Ok(s) => s,
            Err(_) => continue,
        };

        // let input: i32 = input.trim().parse().expect("请输入数字");

        println!("您的输入是 {}", input);

        match input.cmp(&radom) {
            Ordering::Less => println!("小于"),
            Ordering::Greater => println!("大于"),
            Ordering::Equal => println!("差不多"),
        }
    }
}
