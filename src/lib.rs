pub mod api;
pub mod example;

// 父模块无法使用子模块 非 pub 的模块
// 子模块可以使用父模块的内容
mod father {
    pub mod children {

        pub fn drink() {
            // 模块的上一级 super
            super::super::use_father()
        }
    }
}

pub use father::children::drink as use_drink;

pub fn use_father() {
    // use 在 lib 使用
    use_drink();

    // 绝对 crate (单元包调用)
    crate::father::children::drink();
    father::children::drink();
}
