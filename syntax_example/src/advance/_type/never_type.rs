// Rust 有一个叫做 ! 的特殊类型。在类型理论术语中，它被称为 empty type，因为它没有值。
// 我们更倾向于称之为 never type。这个名字描述了它的作用：在函数从不返回的时候充当返回值。

fn use_never_type(guess: String) -> ! {
    // 可以在 ! 中使用 panic 或者 loop 来对应 nerver type
    // loop 可以使用 continue 结束是因为 continue 并不真正返回一个值；或者什么都不返回
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,

            // never type 的另一个用途是 panic!。
            Err(_) => panic!("错误"),
        };
    }
}
