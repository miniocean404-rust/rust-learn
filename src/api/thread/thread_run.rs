use std::{thread, time::Duration};

// 等待线程完毕
fn await_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("子线程：第 {} 个数", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 等待子线程执行完后才向下执行代码
    handle.join().unwrap();

    for i in 1..5 {
        println!("主：第 {} 个数", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// 闭包获取所有权
fn move_thread() {
    let v = vec![1, 2, 3];
    // move 获取闭包使用外部变量的所有权
    // 闭包获取所有权后主线程无法使用这个变量，因为没有了所有权
    let handle = thread::spawn(move || println!("我是一个 vec {:?}", v));

    handle.join().unwrap();
}
