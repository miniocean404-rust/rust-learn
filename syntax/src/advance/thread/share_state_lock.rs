// Parallel:程序的不同部分同时运行

// 多线程导致的问题
//  竞争状态，线程以不一致的顺序访问数据或资源死锁，
//  两个线程彼此等待对方使用完所持有的资源，线程无法继续

// RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T> 的相似性
// 你可能注意到了，因为 counter 是不可变的，不过可以获取其内部值的可变引用；
// 这意味着 Mutex<T> 提供了内部可变性，就像 Cell 系列类型那样。

// 另一个值得注意的细节是 Rust 不能避免使用 Mutex<T> 的全部逻辑错误。
// 回忆一下使用 Rc<T> 就有造成引用循环的风险，这时两个 Rc<T> 值相互引用，造成内存泄漏。
// 同理，Mutex<T> 也有造成 死锁（deadlock） 的风险。
// 这发生于当一个操作需要锁住两个资源而两个线程各持一个锁，这会造成它们永远相互等待。

use std::{
    rc::Rc,
    sync::{
        mpsc::{self},
        Arc, Mutex,
    },
    thread,
};

pub fn use_thread() {}

// Go 语言的名言:不要用共享内存来通信,要用通信来共享内存。
// 下面例子就是 go 不建议的 共享内存来通信
// Mutex 是 mutual exclusion (互斥锁) 的简写在同一时刻，
// Mutex 只允许一个线程来访问某些数据

// Rc<T> 并不能安全的在线程间共享。
// 当 Rc<T> 管理引用计数时，它必须在每一个 clone 调用时增加计数，并在每一个克隆被丢弃时减少计数。
// Rc<T> 并没有使用任何并发原语，来确保改变 [计数的操作] 不会被其他线程打断。
// 在计数出错时可能会导致诡异的 bug，比如可能会造成内存泄漏，或在使用结束之前就丢弃一个值。
// 我们所需要的是一个完全类似 Rc<T>，又以一种线程安全的方式改变引用计数的类型。

// Arc 类似 Rc Arc 是原子操作 Rc 只能用于单线程（atomically reference counted）

// 你可能会好奇为什么不是所有的原始类型都是原子性的？
// 为什么不是所有标准库中的类型都默认使用 Arc<T> 实现？
// 原因在于线程安全带有性能惩罚，我们希望只在必要时才为此买单。
// 如果只是在单线程中对值进行操作，原子性提供的保证并无必要，代码可以因此运行的更快。
fn share_state_thread() {
    let count = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 1..10 {
        let count = Arc::clone(&count);

        let handle = thread::spawn(move || {
            let mut num = count.lock().unwrap();

            *num += 1;
        });

        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap()
    }

    println!("结果 {}", *count.lock().unwrap())
}

// 简单使用
fn use_lock() {
    let m = Mutex::new(5);

    {
        // lock 阻塞获取锁 并且 lock 返回值实现了 Drop trait 所以走出作用域自动释放
        // lock 返回值实现了 Deref 所以可以获取这个值的指针
        let mut num = m.lock().unwrap();

        *num = 6;
    }

    println!("num{:?}", m)
}
