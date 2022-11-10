// 可能会有这样很长的类型 Box<dyn Fn() + Send + 'static>
// 这时候就可以使用 type alise = Box<dyn Fn() + Send + 'static>

// 类型别名（type alias）的能力，使用 type 关键字来给予现有类型另一个名字。：

// 这意味着 Kilometers 是 i32 的 同义词（synonym）；
// 不同于示例 19-15 中创建的 Millimeters 和 Meters 类型。
// Kilometers 不是一个新的、单独的类型。Kilometers 类型的值将被完全当作 i32 类型值来对待：
// 可以将 i32 与 Kilometers 相加，也可以将 Kilometers 传递给获取 i32 参数的函数。

// 但通过这种手段无法获得上一部分讨论的 newtype 模式所提供的类型检查的好处。

pub fn use_type_alise() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}
