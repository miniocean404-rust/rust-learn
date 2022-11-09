// Concurrent:程序的不同部分之间独立的执行
// Parallel:程序的不同部分同时运行

// 多线程导致的问题
//  竞争状态，线程以不一致的顺序访问数据或资源死锁，
//  两个线程彼此等待对方使用完所持有的资源，线程无法继续

use std::{
    sync::{
        mpsc::{self, TryRecvError},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub fn use_thread() {}

// Go 语言的名言:不要用共享内存来通信,要用通信来共享内存。
// 下面例子就是 go 不建议的 共享内存来通信
// Mutex 是 mutual exclusion (互斥锁) 的简写在同一时刻，
// Mutex 只允许一个线程来访问某些数据

// Arc 类似 Rc Arc 是原子操作 Rc 只能用于单线程
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

// Channel 包含:发送端、接收端
// 调用发送端的方法,发送数据接收端会检查和接收到达的数据
// 如果发送端、接收端中任意一端被丢弃了,那么Channel就“关闭”了

// 使用mpsc:channel函数来创建Channel
// mpsc表示multiple producer,single consumer(多个生产者、一个消费者)
// 返回一个tuple(元组)：里面元素分别是发送端、接收端
fn channel_message_send() {
    let (send, receiver) = mpsc::channel();

    let clone_send = mpsc::Sender::clone(&send);

    thread::spawn(move || {
        let message = vec![String::from("hi"), String::from("hi"), String::from("hi")];

        for i in message {
            send.send(i).unwrap();
            thread::sleep(Duration::from_millis(1))
        }

        let val = String::from("hi");
        send.send(val).unwrap();

        let clone_val = String::from("hi clone");
        clone_send.send(clone_val).unwrap()
    });

    // 阻塞等到消息到来
    let rec = receiver.recv().unwrap();

    // 需要循环获取消息处理，异步的
    let rec2 = receiver.try_recv();

    let await_val = match rec2 {
        Ok(v) => v,
        Err(err) => err.to_string(),
    };

    for rec in receiver {
        println!("获取的消息 {}", rec)
    }

    println!("获取发送来的值 {} {}", rec, await_val)
}

// 闭包获取所有权
fn move_thread() {
    let v = vec![1, 2, 3];
    // move 获取闭包使用外部变量的所有权
    // 闭包获取所有权后主线程无法使用这个变量，因为没有了所有权
    let handle = thread::spawn(move || println!("我是一个 vec {:?}", v));

    handle.join().unwrap();
}

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
