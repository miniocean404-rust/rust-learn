use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    thread::{self, sleep},
    time::{Duration, Instant},
};

struct ReadFile {
    is_finish: bool,
}

impl ReadFile {
    fn new() -> ReadFile {
        ReadFile { is_finish: false }
    }
}

impl Future for ReadFile {
    type Output = String;

    // tokie 反应器监听系统 io 的异步事件，当系统事件反应时，通知执行器运行  Future poll 的值
    // 当被第一次轮询时，值还没有完成，会放置到队列 Waker 中
    // 当任务完成时，Waker 有个 wake 方法可以通知执行器 关联的任务可以唤醒了, Tokio 执行器就会通知 Future 再次执行 poll
    // 当任务完成时调用 poll
    // poll (轮询)

    // Pin 代表被单独放到内存的某个空间方便异步任务执行完后，从队列中回归正常队列时可以拿到这个结构体
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker().clone();

        println!("finish 1 {}", self.is_finish);

        if !self.is_finish {
            thread::spawn(move || {
                let time = Instant::now() + Duration::from_millis(2000);
                let dutation = time - Instant::now();

                sleep(dutation);

                // self.is_finish = true;
                // waker().wake() 通知 tokio 异步执行器，任务已经准备好了
                waker.wake();
            });

            self.is_finish = true;
            Poll::Pending
        } else {
            Poll::Ready(String::from("我准备好了"))
        }
    }
}

// 实现了 Future 的才可以 .await。 async 关键字自动为函数返回值加上 Future
#[tokio::main]
pub async fn fulfill_future() {
    let _t1 = tokio::spawn(async {
        let f1 = ReadFile::new();

        println!("{:?}", f1.await)
    });

    let _ = tokio::join!(_t1);
}
