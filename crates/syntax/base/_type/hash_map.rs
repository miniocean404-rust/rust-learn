#![allow(warnings)]

use std::collections::HashMap;

fn use_hash_map() {
    // 创建空 HashMap
    // HashMap  是同构的
    let mut hash: HashMap<&str, &str> = HashMap::new();

    hash.insert("k", "v");

    // 替换 value
    hash.insert("k", "v1");

    // 如果不存在 key 则添加 value
    hash.entry("key").or_insert("value");

    match hash.get(&"key") {
        Some(v) => println!("{}", v),
        None => println!("空"),
    }

    let str_vec: Vec<String> = vec![String::from("1"), String::from("2")];
    let type_number = vec![1, 2];

    // zip 将 两个迭代器里的内容合并为一个
    // 对于 栈类型的值 所有权还在本身，对于堆的数据 所有权会转义给 HashMap（但是如果是 值的引用 则不会导致源变量所有权转移）
    let source: HashMap<_, _> = type_number.iter().zip(str_vec.iter()).collect();

    // 遍历
    for (key, value) in &hash {}
}
