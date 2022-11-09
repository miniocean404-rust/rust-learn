// 不安全 Rust 有两个被称为 裸指针（raw pointers）的类似于引用的新类型。
// 和引用一样，裸指针是不可变或可变的，分别写作 *const T 和 *mut T。
// 这里的星号不是解引用运算符；它是类型名称的一部分。在裸指针的上下文中，不可变 意味着指针解引用之后不能直接赋值。
// 不安全指针为了和 C 进行接口、构建借用检查器无法理解的安全抽象

// 裸指针与引用和智能指针的区别在于
//     允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
//     不保证指向有效的内存
//     允许为空
//     不能实现任何自动清理功能

use core::slice;

// 这里没有引入 unsafe 关键字。可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针，稍后便会看到。
// as 将不可变和可变引用强转为对应的裸指针类型。
fn raw_pointer() {
    let mut num = 5;

    // 如果尝试同时创建 num 的不可变和可变引用，将无法通过编译，因为 Rust 的所有权规则不允许在拥有任何不可变引用的同时再创建一个可变引用。
    // 通过裸指针，就能够同时创建同一地址的可变指针和不可变指针，若通过可变指针修改数据，则可能潜在造成数据竞争
    let r1 = &num as *const i32; // 引用不可变裸指针
    let r2 = &mut num as *mut i32; // 引用可变裸指针

    // 可以在安全代码中创建裸指针，不过不能 解引用 裸指针和读取其指向的数据。现在我们要做的就是对裸指针使用解引用运算符 *，这需要一个 unsafe 块
    // 下面就是在 解引用裸指针
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // 创建一个指向任意内存地址的裸指针。
    // 尝试使用任意内存是未定义行为：此地址可能有数据也可能没有，编译器可能会优化掉这个内存访问，或者程序可能会出现段错误（segmentation fault）。
    // 通常没有好的理由编写这样的代码，不过却是可行的：
    let address = 0x012345usize;
    let r = address as *const i32;
}

// 不安全函数体也是有效的 unsafe 块，所以在不安全函数中进行另一个不安全操作时无需新增额外的 unsafe 块。
fn unsafe_fn() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // 按理说这个函数可以切片这个变量的两个部分，但是 Rust 的安全特性不允许同一个变量使用两次，所以使用 unsafe 进行操作
    // (&mut slice[..mid], &mut slice[mid..])

    // from_raw_parts_mut 本身就是不安全函数
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn painc_raw_pointer() {
    let address = 0x01234usize;
    let r = address as *mut i32;

    // 因为获取的是一个地址，对一个位置的地址操作就很可能导致崩溃
    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
}
