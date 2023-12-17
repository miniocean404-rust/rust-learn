use std::fmt::Display;

pub trait UseTrait {
    fn trait_test(&self) -> String;

    // 设置 trait 默认实现 impl 中不需要写对应的实现（也可以重写覆盖）
    fn trait_test1(&self) -> String {
        String::from("1")
    }

    // 也可以直接调用没有实现的方法
    fn trait_test2(&self) -> String {
        self.trait_test()
    }
}

pub struct Articel {
    pub title: String,
}

// 实现 trait 也就是实现接口
impl UseTrait for Articel {
    fn trait_test(&self) -> String {
        // format!("{}", self.title);
        self.title.to_string()
    }
}

// 实现 某个 trait 的参数类型
pub fn notify(item: impl UseTrait) {
    println!("{}", item.trait_test())
}

pub fn notify3(item: impl UseTrait + Display) {
    println!("{}", item.trait_test())
}

// trait bound 绑定 trait 类型
pub fn notify1<T: UseTrait>(item: T) {
    println!("{}", item.trait_test())
}

// 要求实现了 UseTrait 和 Display 两个类型
pub fn notify2<T: UseTrait + Display>(item: T) {
    println!("{}", item.trait_test())
}

// 使用 where 优化代码观看形式
pub fn notify4<T>(item: T)
where
    T: UseTrait + Display,
{
    println!("{}", item.trait_test())
}

// impl 表示返回类型时候 里面不能有 用 if 判断的返回多种 trait 实现的类型
pub fn notify5() -> impl UseTrait {
    Articel {
        title: String::from(""),
    }
}

// 需要引入 UseTrait
// use learn_base::api::use_trait::{Articel, UseTrait};

// fn main() {
//     let articel = Articel {
//         title: String::from("_"),
//     };

//     println!("{}", articel.trait_test())
// }

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

// 为结构体中 T 泛型实现方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 约束 只有实现 Display + PartialOrd trait 的才有这个方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x == self.y {
            println!("y {}", self.y);
        } else {
            println!("x {}", self.x);
        }
    }
}

// 覆盖实现：为所有实现了 Display trait 调用 ToString trait 的方法
trait ToString {
    fn to_string1(&self) -> String;
}

impl<T: Display> ToString for T {
    fn to_string1(&self) -> String {
        todo!()
    }
}
