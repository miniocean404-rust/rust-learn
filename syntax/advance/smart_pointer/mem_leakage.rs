// Rust 的内存安全性保证使其难以意外地导致内存泄漏（memory leak），
// 但并不是不可能。与在编译时拒绝数据竞争不同， Rust 并不保证完全地避免内存泄漏，这意味着内存泄漏在 Rust 被认为是内存安全的。
// 这一点可以通过 Rc<T> 和 RefCell<T> 看出：创建引用循环的可能性是存在的。
// 这会造成内存泄漏，因为每一项的引用计数永远也到不了 0，其值也永远不会被丢弃。

use self::List::{Cons, Nil};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// 可以看到将列表 a 修改为指向 b 之后，a 和 b 中的 Rc<List> 实例的引用计数都是 2。
// 在 main 的结尾，Rust 丢弃 b，这会 b Rc<List> 实例的引用计数从 2 减为 1。(这个 1 是因为 a 还引用着)
// 然而，b Rc<List> 不能被回收，因为其引用计数是 1 而不是 0。
// 接下来 Rust 会丢弃 a 将 a Rc<List> 实例的引用计数从 2 减为 1。这个实例也不能被回收，因为 b Rc<List> 实例依然引用它，所以其引用计数是 1。(这个 1 是因为 b 还引用着)
// 这些列表的内存将永远保持未被回收的状态。
// 如果取消最后 println! 的注释并运行程序，Rust 会尝试打印出 a 指向 b 指向 a 这样的循环直到栈溢出。
fn cycle_pointer_memory_leak() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a 初始化 rc 数量 = {}", Rc::strong_count(&a));
    println!("a 下一个值 = {:?}", a.tail());

    // b 列表包含 a 的指针
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("在创建 b 后 a rc 数量 = {}", Rc::strong_count(&a));
    println!("b 初始化 rc 数量 = {}", Rc::strong_count(&b));
    println!("b 下一个值 = {:?}", b.tail());

    // a 的列表包含 b 的指针
    if let Some(link) = a.tail() {
        // Nil 改为指向 b
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("在改变 a 后 b rc 数量 = {}", Rc::strong_count(&b));
    println!("在改变 a 后 a rc 数量  = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

// 避免引用循环：将 Rc<T> 变为 Weak<T>
// 也可以通过调用 Rc::downgrade 并传递 Rc<T> 实例的引用来创建其值的 弱引用（weak reference）。
// 调用 Rc::downgrade 时会得到 Weak<T> 类型的智能指针。
// 不同于将 Rc<T> 实例的 strong_count 加 1，调用 Rc::downgrade 会将 weak_count 加 1。
// Rc<T> 类型使用 weak_count 来记录其存在多少个 Weak<T> 引用，类似于 strong_count。
// 其区别在于 weak_count 无需计数为 0 就能使 Rc<T> 实例被清理。

// 强引用代表如何共享 Rc<T> 实例的所有权，但弱引用并不属于所有权关系。
// 他们不会造成引用循环，因为任何弱引用的循环会在其相关的强引用计数为 0 时被打断

// 因为 Weak<T> 引用的值可能已经被丢弃了，为了使用 Weak<T> 所指向的值，我们必须确保其值仍然有效。
// 为此可以调用 Weak<T> 实例的 upgrade 方法，这会返回 Option<Rc<T>>。
// 如果 Rc<T> 值还未被丢弃，则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None。
// 因为 upgrade 返回一个 Option<Rc<T>>所以它不会返回非法指针

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn avoid_cycle_pointer() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    // upgrade 增加 Weak 计数 downgrade 减少计数
    // leaf 新建了一个空的 Weak 引用实例，当尝试使用 upgrade 方法获取 leaf 的父节点引用时，会得到一个 None 值。
    println!("leaf 父亲 = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    // Rc::downgrade 函数从 branch 中的 Rc<Node> 值创建了一个指向 branch 的 Weak<Node> 引用。
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
