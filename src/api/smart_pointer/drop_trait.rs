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
    // 离开作用域删除智能指针的数据
    let a = CustomSmartPoint {
        data: String::from(""),
    };

    //  提前手动清理不调用 Drop trait 的 drop
    drop(a);
}
