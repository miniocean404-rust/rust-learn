#![allow(warnings)]
// 提前定义画的 trait 等待某些组件去实现
pub trait Draw {
    fn draw(&self);
}

// 屏幕结构体
pub struct Screen {
    // dyn 代表动态值，只要实现了 Draw 这个 trait 就可以
    // 不用泛型是泛型只能指明一种类型
    // 限制：
    // 1. dyn trait 不能是泛型，因为它不是对象安全的，它无法在运行时确定泛型参数的具体类型。参数也不能是 Self
    // 2. 方法签名：方法不能有 Self 类型的返回值，除非它们返回 Self 的引用或指针。
    // 3. Sized 约束：trait 不能要求实现它的类型是 Sized。

    //trait 对象执行动态派发
    // 静态派发 (static dispatch)
    // 将 trait 约束作用于泛型时，Rust 编译器会执行单态化:
    //      编译器会为我们用来替换泛型类型参数的每一个具体类型生成对应函数和方法的非泛型实现。
    //      通过单态化生成的代码会执行静态派发 (static dispatch), 在编译过程中确定调用的具体方法
    // 动态派发 (dynamic dispatch) :
    //      无法在编译过程中确定你调用的究竟是哪一种方法
    //      编译器会产生额外的代码以便在运行时找出希望调用的方法
    // 使用 trait 对象，会执行动态派发:
    //      产生运行时开销
    //      阻止编译器内联方法代码，使得部分优化操作无法进行
    pub components: Vec<Box<dyn Draw>>,
}

// 在屏幕上画的方法
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 实现 Draw 的组件
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

// 实现 Draw 的组件
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

pub fn use_oo() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// trait 对象必须保证是安全的
// 只能把满足对象安全(object-safe)的 trait 转化为 trait 对象 Rust采用一系列规则来判定某个对象是否安全，只需记住两条：
//      方法的返回类型不是Self
//      方法中不包含任何泛型类型参数

pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct Screen1 {
    // 因为 Clone 返回了 Self 所以 这个 trait 对象就是不安全的
    // pub components: Vec<Box<dyn Clone>>,
}
