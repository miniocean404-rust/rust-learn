// use learn_base::api::use_struct;
// use learn_base::example::read_line;

// self 表示引入 use learn_base
// use learn_base::{self, api::use_struct, example::read_line};

// 导入全部
// use neon::prelude::*; 导入全部

use learn_base::example::server::single_server::use_single_server;

fn main() {
    use_single_server()
}
