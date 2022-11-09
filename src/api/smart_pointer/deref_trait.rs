use std::ops::Deref;

// 解引用和可变性 DerefMut
// 可使用 DerefMut trait 重载可变引用的 * 运算符
// 在类型和trait在下列三种情况发生时，Rust会执行 deref coercion:
//  -当T:Deref<.Target-=U>,允许&T转换为&U
//  -当T:DerefMut<Target-=U>,允许&mutT转换为&mutU
//  -当T:Deref<Target-=U>,允许&mutT转换为&U

// Deref Trait 相当于 &xx
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

    // 解引用实际会隐式调用 *(y.deref())
    assert_eq!(5, *y);

    // 函数和方法的隐式转换
    // 隐式解引用转化(Deref Coercion)是为函数和方法提供的一种便捷特性
    // 假设 T 实现了Deref trait:
    // - Deref Coercion 可以把 &MyBox<T> 转化为 T 经过 Deref 操作后生成的 &<T>

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
    println!("Hello, {}!", name);
}
