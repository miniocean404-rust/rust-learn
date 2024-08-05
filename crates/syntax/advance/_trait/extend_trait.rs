#![allow(dead_code)]
use std::fmt::{Display, Formatter, Result};

// trait 继承
// 场景：
// 有时我们可能会需要某个 trait 使用另一个 trait 的功能。
// 在这种情况下，需要能够依赖相关的 trait 也被实现。这个所需的 trait 是我们实现的 trait 的 父 trait

// 实现 OutlinePrint trait，继承 Display 的功能
// 因为指定了 OutlinePrint 需要 Display trait，则可以在 outline_print 中使用 to_string， 其会为任何实现 Display 的类型自动实现。
// 如果不在 trait 名后增加 : Display 并尝试在 outline_print 中使用 to_string，则会得到一个错误说在当前作用域中没有找到用于 &Self 类型的方法 to_string。

// 在 outline_print 的实现中，因为希望能够使用 Display trait 的功能
// 需要说明 OutlinePrint 只能用于同时也实现了 Display 并提供了 OutlinePrint 需要的功能的类型。
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// OutlinePrint 想继承别的 trait 那么实现它的结构体等就需要实现其父的 trait
impl OutlinePrint for Point {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
