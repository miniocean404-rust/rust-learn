// trait 中至少有一个方法中包含编译器无法验证的不变式（invariant）时 trait 是不安全的。
// 可以在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe，同时 trait 的实现也必须标记为 unsafe
// unsafe trait 只能在 unsafe 代码块中实现

// unsafe trait
unsafe trait Foo {}

// 实现
unsafe impl Foo for i32 {
    // method implementations go here
}

// 何时使用不安全代码
// 使用 unsafe 来进行这五个操作（超能力）之一是没有问题的，甚至是不需要深思熟虑的，
// 不过使得 unsafe 代码正确也实属不易，因为编译器不能帮助保证内存安全。
// 当有理由使用 unsafe 代码时，是可以这么做的，通过使用显式的 unsafe 标注可以更容易地在错误发生时追踪问题的源头。
