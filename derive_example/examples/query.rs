use derive_example::query;

// 文件夹名称必须是 examples
// 运行 cargo run --example query
fn main() {
    // query!(SELECT * FROM users WHERE age > 10);
    query!(SELECT * FROM users u JOIN (SELECT * from profiles p) WHERE u.id = p.id and u.age > 10);
    hello(); // 调用 query 宏的 hello 函数，输出 Hello world!
}
