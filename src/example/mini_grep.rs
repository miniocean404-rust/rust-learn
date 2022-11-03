use crate::example::mini_grep_lib::get_file_content;
use crate::example::mini_grep_lib::Config;
use std::env;
use std::process;

pub fn use_grep() {
    // env::args().collect() 获取的参数 ["target\\debug\\learn_base.exe", "1234"]
    let args: Vec<String> = env::args().collect();

    // |err| 闭包的参数
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("解析参数有问题: {}", err);
        process::exit(0);
    });
    // println!("参数值：{:?}{:?}", config.query, config.filename);

    if let Err(e) = get_file_content(config) {
        // 可使用命令 cargo run > log.txt 将 标准输入的东西输出到文件
        println!("运行错误 {}", e);
        process::exit(1);
    };
}
