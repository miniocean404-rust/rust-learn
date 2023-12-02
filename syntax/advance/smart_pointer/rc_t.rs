// 引用计数指针,启用多所有权
// 为了启用多所有权，Rust 有一个叫做 Rc<T> 的类型。其名称为 引用计数（reference counting）的缩写
// 引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。
// 注意 Rc<T> 只能用于单线程场景

use std::rc::Rc;
use List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// 不同于获取 a 的所有权，这里会克隆 a 所包含的 Rc<List>，这会将引用计数从 1 增加到 2 并允许 a 和 b 共享 Rc<List> 中数据的所有权。
// 创建 c 时也会克隆 a，这会将引用计数从 2 增加为 3。
// 每次调用 Rc::clone，Rc<List> 中数据的引用计数都会增加，直到有零个引用之前其数据都不会被清理。

pub fn use_rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("创建 A 后的数量 = {}", Rc::strong_count(&a));

    {
        let d = Cons(4, Rc::clone(&a));
        // 其值可以通过调用 Rc::strong_count 函数获得。
        // 这个函数叫做 strong_count 而不是 count 是因为 Rc<T> 也有 weak_count；
        // 在 “避免引用循环：将 Rc<T> 变为 Weak<T>” 部分会讲解 weak_count 的用途。
        // https://kaisery.github.io/trpl-zh-cn/ch15-06-reference-cycles.html#preventing-reference-cycles-turning-an-rct-into-a-weakt

        // Drop trait 的实现当 Rc<T> 值离开作用域时自动减少引用计数。
        println!("创建 D 后的数量  = {}", Rc::strong_count(&a));
    }

    // 也可以调用 a.clone() 而不是 Rc::clone(&a)，
    // 不过在这里 Rust 的习惯是使用 Rc::clone。Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。
    // Rc::clone 只会增加引用计数，这并不会花费多少时间。深拷贝可能会花费很长时间。
    // 通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。当查找代码中的性能问题时，
    // 只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。
    let b = Cons(3, Rc::clone(&a));
    println!("创建 B 后的数量 = {}", Rc::strong_count(&a));

    let c = Cons(4, Rc::clone(&a));
    println!("创建 C 后的数量 = {}", Rc::strong_count(&a));
}
