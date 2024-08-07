#![allow(warnings)]
// 因为 Rust 需要知道例如应该为特定类型的值分配多少空间,这样的信息其类型系统的一个特定的位置可能令人迷惑：这就是 动态大小类型（dynamically sized types）的概念。
// 这有时被称为 “DST” 或 “unsized types”，这些类型允许我们处理只有在运行时才知道大小的类型

// str 是一个 DST；直到运行时我们都不知道字符串有多长。因为直到运行时都不能知道其大小，也就意味着不能创建 str 类型的变量，也不能获取 str 类型的参数。
// 看一下这些代码，他们不能工作：

// Rust 需要知道应该为特定类型的值分配多少内存，同时所有同一类型的值必须使用相同数量的内存。
// 如果允许编写这样的代码，也就意味着这两个 str 需要占用完全相同大小的空间，
// 不过它们有着不同的长度。这也就是为什么不可能创建一个存放动态大小类型的变量的原因。

// 那么该怎么办呢？你已经知道了这种问题的答案：s1 和 s2 的类型是 &str 而不是 str。

// 所以虽然 &T 是一个储存了 T 所在的内存位置的单个值，
// &str 则是 两个 值：str 的地址和其长度。这样，&str 就有了一个在编译时可以知道的大小：它是 usize 长度的两倍。
// 也就是说，我们总是知道 &str 的大小，而无论其引用的字符串是多长。
// 这里是 Rust 中动态大小类型的常规用法：他们有一些额外的元信息来储存动态信息的大小。
// 这引出了动态大小类型的黄金规则：必须将动态大小类型的值置于某种指针之后。
#[allow(unused_must_use)]
pub fn use_size_trait() {
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";

    // 可以将 str 与所有类型的指针结合：
    //      比如 Box<str> 或 Rc<str>。
    // 事实上，之前我们已经见过了，不过是另一个动态大小类型：trait。
    // 每一个 trait 都是一个可以通过 trait 名称来引用的动态大小类型。
    // 在第十七章 “为使用不同类型的值而设计的 trait 对象” 部分，我们提到了为了将 trait 用于 trait 对象，必须将他们放入指针之后，
    // 比如 &dyn Trait 或 Box<dyn Trait>（Rc<dyn Trait> 也可以）。

    // 为了处理 DST，Rust 有一个特定的 trait 来决定一个类型的大小是否在编译时可知：
    //      这就是 Sized trait。这个 trait 自动为编译器在编译时就知道大小的类型实现。
    // 另外，Rust 隐式的为每一个泛型函数增加了 Sized bound。也就是说，对于如下泛型函数定义：
    fn generic<T>(t: T) {
        // --snip--
    }

    // 实际上被当作如下处理：
    fn generic1<T: Sized>(t: T) {
        // --snip--
    }

    // 泛型函数默认只能用于在编译时已知大小的类型。然而可以使用如下特殊语法来放宽这个限制：
    // ?Sized 上的 trait bound 意味着 “T 可能是也可能不是 Sized”
    // 同时这个注解会覆盖泛型类型必须在编译时拥有固定大小的默认规则。
    // 这种意义的 ?Trait 语法只能用于 Sized ，而不能用于任何其他 trait。

    // 另外注意我们将 t 参数的类型从 T 变为了 &T：因为其类型可能不是 Sized 的，所以需要将其置于某种指针之后。在这个例子中选择了引用
    fn generic2<T: ?Sized>(t: &T) {
        // --snip--
    }
}
