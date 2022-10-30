use std::string;

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

// 需要引入 UseTrait
// use learn_base::api::use_trait::{Articel, UseTrait};

// fn main() {
//     let articel = Articel {
//         title: String::from("_"),
//     };

//     println!("{}", articel.trait_test())
// }
