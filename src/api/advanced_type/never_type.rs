// Rust 有一个叫做 ! 的特殊类型。在类型理论术语中，它被称为 empty type，因为它没有值。
// 我们更倾向于称之为 never type。这个名字描述了它的作用：在函数从不返回的时候充当返回值。

pub fn bar() -> ! {
    // --snip--
}

fn use_never_type(guess: String) {
    // guess 必须既是整型 也是 字符串，而 Rust 要求 guess 只能是一个类型。那么 continue 返回了什么呢
    // 描述 ! 的行为的正式方式是 never type 可以强转为任何其他类型。允许 match 的分支以 continue 结束是因为 continue 并不真正返回一个值；

    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // continue 要运行在 loop 中
            // 返回值要求返回为 同一个类型 ，当 Err 为字符串时是不允许编译的
            // 但是 continue 的值是 !。也就是说，当 Rust 要计算 guess 的类型时，它查看这两个分支。前者是 u32 值，而后者是 ! 值。因为 ! 并没有一个值，Rust 决定 guess 的类型是 u32
            // ! 的行为的正式方式是 never type 可以强转为任何其他类型。允许 match 的分支以 continue 结束是因为 continue 并不真正返回一个值；相反它把控制权交回上层循环，所以在 Err 的情况，事实上并未对 guess 赋值
            Err(_) => continue,

            // never type 的另一个用途是 panic!。
            Err(_) => panic!("错误"),
        };
    }

    // 最后一个有着 ! 类型的表达式是 loop：
    loop {
        print!("and ever ");
    }
}
