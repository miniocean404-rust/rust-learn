// 关联类型

pub trait Iterator {
    // Item 是一个占位类型，同时 next 方法定义表明它返回 Option<Self::Item> 类型的值。
    // 这个 trait 的实现者会指定 Item 的具体类型，然而不管实现者指定何种类型,
    // next 方法都会返回一个包含了此具体类型值的 Option。
    // 相对于泛型只需要指定一次类型
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
