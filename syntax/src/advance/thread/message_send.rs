use std::{
    sync::mpsc::{self},
    thread,
    time::Duration,
};

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
