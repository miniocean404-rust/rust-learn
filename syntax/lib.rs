pub mod api;
pub mod base;
pub mod example;

// pub use 导出使得用户调用更方便的导出
// 正常开发会分为很多层，调用者需要挨个层次导入 比如：a::b::c::d;
// pub use 可以将导入的东西再次导出 变成 a::d;
pub use father::children::drink as use_drink;
// main.rs 中直接 learn_base:: use_pub_use；进行调用
pub use self::api::publish::pub_use::use_pub_use;

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

pub fn use_father() {
    // use 在 lib 使用
    use_drink();

    // 绝对 crate (单元包调用)
    crate::father::children::drink();
    father::children::drink();
}
