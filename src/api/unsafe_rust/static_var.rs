// 目前为止全书都尽量避免讨论 全局变量（global variables），Rust 确实支持他们，
// 不过这对于 Rust 的所有权规则来说是有问题的。如果有两个线程访问相同的可变全局变量，则可能会造成数据竞争。

// 通常静态变量的名称采用 SCREAMING_SNAKE_CASE 写法。
// 静态变量只能储存拥有 'static 生命周期的引用，这意味着 Rust 编译器可以自己计算出其生命周期而无需显式标注。
// 访问不可变静态变量是安全的。
// 必须显示声明类型

// 全局变量在 Rust 中被称为 静态（static）变量。

static HELLO_WORLD: &str = "Hello, world!";

pub fn use_static() {
    println!("name is: {}", HELLO_WORLD);
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// 修改静态变量：
// 静态变量 和 常量 的区别
// 静态变量：固定的内存地址，使用总会得到相同的数据，并且它是可以改变的，访问修改静态变量是不安全的 unsafe 的
// 常量：允许使用他们的时候对数据进行复制
// 多线程时大概率导致数据竞争，尽可能使用只能指针
pub fn change_static() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
