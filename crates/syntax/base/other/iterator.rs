#![allow(warnings)]

pub fn use_iterator() {
    let v1 = vec![1, 2, 3];

    // Rust 迭代器是惰性的，在没有调用时候没有任何效果
    // 迭代器 Trait 中的 Type item; 实际上是 一个 next 方法的返回值类型，也是迭代器的返回类型
    let iter = v1.iter(); // iter 在不可变引用中创建迭代器
                          // let iter1 = v1.into_iter(); // 会将调用者移动新的作用域内，并获得所有权
                          // let iter2 = v1.iter_mut(); // 迭代可变引用

    // let val = v1.iter().next();

    for val in iter {
        println!("{}", val)
    }
}

// 消耗迭代器的方法, 每个方法底层都是使用 next
pub fn consume_iterator() {
    let v1 = vec![1, 2, 3];
    let iter = v1.iter();

    // sum 获取变量所有权，并结合所有值后一起返回
    let total: i32 = iter.sum();

    println!("{}", total)
}

// 迭代器适配器
pub fn adapter_iterator() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 4];
    let iter = v1.iter();
    let iter1 = v2.into_iter();

    // sum 获取变量所有权，并结合所有值后一起返回
    let total = iter.map(|x| x + 1);
    let is_have = iter1.filter(|x| *x == 3);

    // _ 代表迭代器推断类型
    let val: Vec<_> = total.collect();
    let val1: Vec<_> = is_have.collect();

    println!("{:?} {:?}", val, val1)
}

// 自己实现 Iterator
// Rust 中的迭代器没有运行时开销，都会将迭代器 API 内容转换成底层 for in 的汇编代码
struct Count {
    count: i32,
}

impl Count {
    fn new() -> Count {
        Count { count: 0 }
    }
}

impl Iterator for Count {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 3 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn custom_iterator() {
    let mut count = Count::new();

    count.next();
    count.next();
    count.next();
    count.next();
    count.next();

    match count.next() {
        Some(v) => {
            println!("{}", v)
        }
        None => {
            println!("{}", -1)
        }
    }
}
