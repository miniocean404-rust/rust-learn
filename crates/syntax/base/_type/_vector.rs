#![allow(warnings)]

enum UniteVectorEnum {
    Test(i32),
    Test2(u8),
    Test3(String),
}

fn show_vector() {
    // 只能存放一种数据类型，但是和枚举共同使用可以存放多种类型
    let mut v: Vec<i32> = Vec::new();
    v.push(1);

    let v1 = vec![1, 2, 3];
    let value = v1[1];

    // get 可以处理越界
    let value1 = v1.get(1);

    match value1 {
        Some(v) => {}
        None => {}
    }

    // 遍历
    for item in &v {}

    let v2 = vec![
        UniteVectorEnum::Test(1),
        UniteVectorEnum::Test2(2),
        UniteVectorEnum::Test3(String::from("1")),
    ];
}
