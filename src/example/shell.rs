use std::process::Command;

pub fn use_command() {
    let out = Command::new("yarn")
        .arg("config")
        .args(["list"])
        .output()
        .expect("执行异常");

    let str = String::from_utf8(out.stdout).unwrap_or_else(|err| panic!("err {}", err));

    println!("str {}", str);

    let mut output = Command::new("ls");
    // 将命令作为子进程执行，等待它完成并收集其状态。
    output.status().expect("程序执行失败提示"); // 当前目录无需切换，直接执行

    // 为子进程设置工作目录
    output.current_dir("/"); // 切换到/目录

    // 命令插入环境变量
    Command::new("ls")
        .env("PATH", "/bin")
        .spawn()
        .expect("命令执行异常提示");

    // 命令移除环境变量
    Command::new("ls")
        .env_remove("PATH")
        .spawn()
        .expect("命令执行异常提示");

    Command::new("ls")
        .env_clear()
        .spawn()
        .expect("命令执行异常提示");
}
