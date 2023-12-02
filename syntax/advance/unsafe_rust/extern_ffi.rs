// 使用 extern 函数调用外部代码
// extern，有助于创建和使用 外部函数接口（Foreign Function Interface， FFI）
// "C" 部分定义了外部函数所使用的 应用二进制接口（application binary interface，ABI） —— ABI 定义了如何在汇编语言层面调用此函数。
// "C" ABI 是最常见的，并遵循 C 编程语言的 ABI。

// 调用 C 代码函数
extern "C" {
    fn abs(input: i32) -> i32;
}

fn use_c_ffi() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 还需增加 #[no_mangle] 注解来告诉 Rust 编译器不要 编译(mangle) 此函数的名称。
// Mangling 发生于当编译器将我们指定的函数名修改为不同的名称时，这会增加用于其他编译过程的额外信息，不过会使其名称更难以阅读。
// 每一个编程语言的编译器都会以稍微不同的方式 mangle 函数名，所以为了使 Rust 函数能在其他语言中指定，必须禁用 Rust 编译器的 name mangling。

// 一旦其编译为动态库并从 C 语言中链接(被 C 访问)，call_from_c 函数就能够在 C 代码中访问
// extern 的使用无需 unsafe。

// 提供给 C 调用的 ffi
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
