pub fn _use_log4() {
    // 导入
    // extern crate log;

    log4rs::init_file("log4rs.yaml", Default::default()).expect("");
}
