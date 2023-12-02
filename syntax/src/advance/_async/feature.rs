use std::time::Duration;

use tokio::time::sleep;

#[tokio::main]
pub async fn use_feature() {
    get_feature().await;
}

// .await 实际返回的是 Future 类型的值
async fn get_feature() -> String {
    sleep(Duration::new(2, 1)).await;

    let str = String::from("我是 future 等待的值");

    println!("{}", str);

    str
}
