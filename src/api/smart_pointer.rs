// 智能指针 通常会实现 Deref 和 Drop 这两个 trait
// Deref 允许智能指针的 struct 的实例像引用一样使用
// Drop 允许你自定义智能指针实例走出作用域时的代码

// 标准库常见的智能指针
// BoX<T>:在 heap 堆内存上分配值
// Rc<T>:启用多重所有权的引用计数类型
// Ref<T>和RefMut<T>,通过RefCell<T>访问：在运行时而不是编译时强制借用规则的类型

// 此外
// 内部可变模式(interior mutability pattern) :不可变类型暴露出可修改其内部值的API
// 引用循环(reference cycles) :它们如何泄露内存,以及如何防止其发生。

// 最简单的智能指针，实现了 Deref 和 Drop 这两个 trait
// 是一个栈类型，其内部每个栈有一个指向堆的指针
// 常用场景：
//  在编译时，某类型的大小无法确定。但使用该类型时，上下文却需要知道它的确切大小。
//  当你有大量数据，想移交所有权，但需要确保在操作时数据不会被复制。
//  使用某个值时，你只关心它是否实现了特定的 trait ,而不关心它的具体类型。
use std::ops::Deref;
use List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn box_T() {
    // 基本使用
    let num = Box::new(5);
    println!("{}", num);

    // 常用场景1的示例 递归类型
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// 实现 Deref Trait 使我们可以自定义解引用运算符 * 的行为
// 通过实现 Deref,智能指针可像常规引用一样来处理

struct MyBox<T>(T); // 元组

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // 实现了 Deref trait 就可以使用 * 解引用
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn deref_trait() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, *y);
    // 解引用实际会隐式调用 *(y.deref())

    // 函数和方法的隐式转换
    // 隐式解引用转化(Deref Coercion)是为函数和方法提供的一种便捷特性
    // 假设 T 实现了Deref trait:
    // - Deref Coercion 可以把 T 的引用转化为 T 经过 Deref 操作后生成的引用
    // 当把某类型的引用传递给函数或方法时,但它的类型与定义的参数类型不匹配:
    //   Deref Coercion 就会自动发生
    //   编译器会对 deref 进行一系列调用,来把它转为所需的参数类型在编译时完成,没有额外性能开销
    let m = MyBox::new(String::from("rust"));

    // &m MyBox<String>
    // deref &String String 实现了 Deref
    // deref &str
    hello(&m)
}

fn hello(name: &str) {
    println!("{}", name)
}

// 解引用和可变性 DerefMut
// 可使用 DerefMut trait 重载可变引用的 * 运算符
// 在类型和trait在下列三种情况发生时，Rust会执行 deref coercion:
//  -当T:Deref<.Target-=U>,允许&T转换为&U
//  -当T:DerefMut<Target-=U>,允许&mutT转换为&mutU
//  -当T:Deref<Target-=U>,允许&mutT转换为&U

// Drop trait
// 可以让我们自定义当值将要离开作用域时发生的动作一
// 例如:文件、网络资源释放等
// 任何类型都可以实现 Drop trait
struct CustomSmartPoint {
    data: String,
}

impl Drop for CustomSmartPoint {
    fn drop(&mut self) {
        println!("离开作用域删除智能指针的数据{}", self.data)
    }
}

fn use_drop() {
    let a = CustomSmartPoint {
        data: String::from(""),
    };

    //  提前手动清理不调用 Drop trait 的 drop
    drop(a);
}
