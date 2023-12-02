use std::net::UdpSocket;

pub fn ip_v4_address() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("期待绑定");
    socket.connect("8.8.8.8:80").expect("期待链接");

    match socket.local_addr() {
        Ok(addr) => Some(addr.ip().to_string()),
        Err(_) => None,
    }
}
