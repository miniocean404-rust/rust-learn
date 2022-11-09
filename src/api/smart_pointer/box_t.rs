use List::{Cons, Nil};

// Box<T>
// 最简单的智能指针，实现了 Deref 和 Drop 这两个 trait
// 是一个栈类型，其内部每个栈有一个指向堆的指针
// 常用场景：
//  在编译时，某类型的大小无法确定。但使用该类型时，上下文却需要知道它的确切大小。
//  当你有大量数据，想移交所有权，但需要确保在操作时数据不会被复制。
//  使用某个值时，你只关心它是否实现了特定的 trait ,而不关心它的具体类型。

// 因为 enum 实际上只会使用其中的一个成员，所以Message 值所需的空间等于储存其最大成员的空间大小
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Cons 构造函数的缩写
// Nil 代表无效或缺失的值且没有下一项
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn box_t() {
    // 将栈的数据存储到堆中，离开作用域后释放作用于 box 本身（位于栈上）和它所指向的数据（位于堆上）。
    let num = Box::new(5);
    println!("{}", num);

    // 常用场景1的示例 递归类型
    let list: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
