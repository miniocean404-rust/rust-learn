use std::{thread::sleep, time::Duration};

// .await 才会执行异步任务，且异步任务需要添加 async 字段
#[tokio::main]
pub async fn use_tokio() {
    let t1 = tokio::spawn(async { task(1, 4).await });
    let t2 = tokio::spawn(async { task(2, 2).await });

    let _ = tokio::join!(t1, t2);
}

pub async fn task(task_id: u8, time: u64) -> String {
    sleep(Duration::new(time, 0));

    let str = format!("我是异步任务: {}", task_id);
    println!("{}", str);

    str
}
