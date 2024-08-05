#![allow(dead_code)]

#[derive(Debug)]
pub enum IpAddressType {
    V4(u8, u8, u8, u8),
    V6(String),
    VTest(i32, i32),
    VTest2 { width: i8 },
    VTest3(String),
}

// 枚举方法
impl IpAddressType {
    fn call(&self) {
        // 解引用
        // *&self
    }
}

struct IpAddress {
    address: String,
    kind: IpAddressType,
}

fn get_ip() {
    let ip = IpAddress {
        address: String::from("127.0.0.1"),
        kind: IpAddressType::V4(128, 0, 0, 0),
    };

    let v_test = IpAddressType::VTest(12, 12);
    let v_test2 = IpAddressType::VTest2 { width: 2 };
    let v_test3 = IpAddressType::VTest3(String::from("127.0.0.1"));

    println!("{:#?} {:#?} {:#?}", v_test, v_test2, v_test3);
}
