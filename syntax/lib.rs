pub mod advance;
pub mod base;
pub mod example;

// pub use 导出使得用户调用更方便的导出
// 正常开发会分为很多层，调用者需要挨个层次导入 比如：a::b::c::d;
// pub use 可以将导入的东西再次导出 变成 a::d;
pub use father_create::children_create::drink as use_drink;

// 父模块无法使用子模块 非 pub 的模块
// 子模块可以使用父模块的内容
mod father_create {
    pub mod children_create {
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
    crate::father_create::children_create::drink();

    // 相对当前包
    self::father_create::children_create::drink();
}
