// 内部可变性:
// 内部可变性（Interior mutability）是 Rust 中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的。
// 为了改变数据，该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则。
// 当可以确保代码在运行时会遵守借用规则，即使编译器不能保证的情况，可以选择使用那些运用内部可变性模式的类型。所涉及的 unsafe 代码将被封装进安全的 API 中，而外部类型仍然是不可变的。

// RefCell<T>
// RefCell<T> 代表其数据的唯一的所有权。
// RefCell<T> 正是用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候。
// 类似于 Rc<T>，RefCell<T> 只能用于单线程场景。如果尝试在多线程上下文中使用RefCell<T>，会得到一个编译错误。

// 对于引用和 Box<T>，借用规则的不可变性作用于编译时。对于 RefCell<T>，这些不可变性作用于 运行时。
// 对于引用，如果违反这些规则，会得到一个编译错误。而对于 RefCell<T>，如果违反这些规则程序会 panic 并退出。

// Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
// Box<T> 允许在编译时执行不可变或可变借用检查；
// RefCell<T> 允许在运行时执行不可变或可变借用检查。

// Rc<T> 仅允许在编译时执行不可变借用检查；
// 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。

use std::{cell::RefCell, rc::Rc};

fn not_variable_to_mut() {
    // 根据借用规则，向 a 中 push 数据需要为 mut 变量，在不为 mut 时，从不可变改为可变内部变量使用 RefCell

    // 当创建不可变和可变引用时，我们分别使用 & 和 &mut 语法。
    // 对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法，这属于 RefCell<T> 安全 API 的一部分。
    // borrow 方法返回 Ref<T> 类型的智能指针，borrow_mut 方法返回 RefMut<T> 类型的智能指针。这两个类型都实现了 Deref，所以可以当作常规引用对待。

    // RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。
    // 每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。当 Ref<T> 值离开作用域时，不可变借用计数减一。
    // 就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。
    let a = RefCell::new(vec![1, 2, 3]);

    a.borrow_mut().push(1);
}

// 内部可变 的指针 的多个拥有者
fn multiple_mut_owner() {
    let await_clone = Rc::new(RefCell::new(5));

    let a = Rc::clone(&await_clone);
    let b = Rc::clone(&await_clone);
}
