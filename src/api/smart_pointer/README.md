# 智能指针 trait

通常会实现 Deref 和 Drop 这两个 trait

- Deref 允许智能指针的 struct 的实例像引用一样使用
- Drop 允许你自定义智能指针实例走出作用域时的代码

# 标准库常见的智能指针

- BoX<T>:在 heap 堆内存上分配值
- Rc<T>:启用多重所有权的引用计数类型
- Ref<T>和 RefMut<T>,通过 RefCell<T>访问：在运行时而不是编译时强制借用规则的类型

# 此外

- 内部可变模式(interior mutability pattern) :不可变类型暴露出可修改其内部值的 API
- 引用循环(reference cycles) :它们如何泄露内存,以及如何防止其发生。
