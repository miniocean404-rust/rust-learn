use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
};

use crate::example::server::thread_poll::ThreadPool;

pub fn use_thread_poll_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    println!("链接建立在 http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

// http 请求体
// Method Request-URI HTTP-Version CRLF
// headers CRLF
// message-body

// http 响应体
// HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // 读取请求到 buffer 中
    let _read_num = stream.read(&mut buffer).unwrap();

    // b 将字符串转为字节字符串
    let get = b"GET / HTTP/1.1\r\n";

    // 请求过来的 buffer 如果包含 get
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // 路径是基于当前命令行的位置开始
    let mut path_buffer = PathBuf::new();
    path_buffer.push("./src/example/server/html");
    path_buffer.push(filename);

    let contents = fs::read_to_string(path_buffer).unwrap();

    // 将 200 请求 及 html 响应回去
    // format! 将字符串拼接
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();

    // 刷新此输出流，确保所有中间缓冲的内容都到达目的地
    stream.flush().unwrap();
}
