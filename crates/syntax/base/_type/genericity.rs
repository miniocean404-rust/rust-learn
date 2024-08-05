#![allow(warnings)]

// 泛型使用

pub fn use_genericity<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut max = &list[0];

    for item in list {
        // > 号 对应一个方法 std::cmp::PartialOrd 所有 只有list 实现了 PartialOrd trait 才能用大于号
        if item > max {
            max = item;
        }
    }

    max
}

struct Point<T, U> {
    x: T,
    y: U,
}

enum Rect<T, U> {
    Test(T),
    Err(U),
}

// 在 impl 后放置类型<T, U>，表示在类型 <T, U> 上实现方法
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 针对这个类型有的方法
impl Point<i32, i32> {
    fn x1(&self) -> &i32 {
        &self.x
    }
}

impl<T, U> Point<T, U> {
    fn x2<X, Y>(self, other: Point<X, Y>) -> Point<T, Y> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn use_struct_genericity() {
    let pint = Point { x: 1, y: 2.0 };
}
