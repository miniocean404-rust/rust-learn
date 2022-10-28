fn show_vector() {
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
}
