#![allow(dead_code)]

// 字符串是字节的集合，一些方法能将字节转换为 字符串
fn use_string() {
    let mut s = String::new();

    let str = "toString 后转化为 string 类型";
    let string_type = str.to_string();
    let mut string_type2: String = String::from("我是 string 类型");

    // 添加字符串切片
    string_type2.push_str("我是插入的 切片字符串类型");

    // 添加单个字符, 使用单引号
    string_type2.push('l');

    // 使用 new 类型 + 切片字符串
    // s 的所有权在 内部调用函数后离开了作用域失效了
    let str = s + &string_type2;

    // string 是 vec<u8> 的包装
    println!("{}", str.len());

    // 获取每个字符的字节
    for item in str.bytes() {}

    // 获取每个字符的 Unicode 标量值
    for item in str.chars() {}

    // 获取切片字符串的切片 0-4 个字节，这种使用方法必须 沿着字符串的边缘切割 （1 个字符多少个字节），否则程序报错
    let str2 = &str[0..4];
}
