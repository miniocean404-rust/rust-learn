// use learn_base::api::use_struct;
// use learn_base::example::read_line;

// self 表示引入 use learn_base
// use learn_base::{self, api::use_struct, example::read_line};

use learn_base::example::mini_grep;

fn main() {
    mini_grep::use_grep();
}
