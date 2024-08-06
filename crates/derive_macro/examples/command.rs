use derive_macro::Builder;

// 运行 // 运行 cargo r -p derive_macro --example derive_macro
#[allow(dead_code)]
#[derive(Debug, Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {}
