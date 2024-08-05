#![allow(warnings)]
// !运算符重载
use std::ops::Add;

// Rhs=Self：这个语法叫做 默认类型参数（default type parameters）
// Rhs 是一个泛型类型参数（“right hand side” 的缩写）
// 它用于定义 add 方法中的 rhs 参数。
// 如果实现 Add trait 时不指定 Rhs 的具体类型，Rhs 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型
trait Add1<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn use_reload() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

// 这里有两个存放不同单元值的结构体，Millimeters 和 Meters。（这种将现有类型简单封装进另一个结构体的方式被称为 newtype 模式（newtype pattern）
// 我们希望能够将毫米值与米值相加，并让 Add 的实现正确处理转换。可以为 Millimeters 实现 Add 并以 Meters 作为 Rhs
// 默认参数类型主要用于如下两个方面：
//      扩展类型而不破坏现有代码。
//      在大部分用户都不需要的特定情况进行自定义。

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
