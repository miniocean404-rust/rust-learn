use std::env;
use std::fs;

pub fn use_grep() {
    // ["target\\debug\\learn_base.exe", "1234"]
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("参数值：{:?}{:?}", config.query, config.filename);

    let content = fs::read_to_string(config.filename).expect("读取文件错误");
    println!("文件内容：\r\n{}", content)
}

struct Config {
    filename: String,
    query: String,
}

impl Config {
    // 解析命令行参数
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { filename, query }
    }
}
