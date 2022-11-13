use std::{process::Output, time::Duration};

use tokio::time::sleep;

use std::future::Future;

#[tokio::main]
pub async fn use_feature() {
    get_feature().await;
}

// .await 实际返回的是 Future 类型的值
fn get_feature() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(2, 1));

        let str = String::from("我是 future 等待的值");

        println!("{}", str);

        str
    }
}
