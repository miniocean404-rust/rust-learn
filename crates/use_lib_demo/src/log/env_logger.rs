#![warn(warnings)]

use chrono::Local;
use env_logger::{
    fmt::{Color, Style, StyledValue},
    Builder,
};
use log::Level;
use std::{
    fmt,
    io::Write,
    sync::atomic::{AtomicUsize, Ordering},
};

struct Padded<T> {
    value: T,
    width: usize,
}

impl<T: fmt::Display> fmt::Display for Padded<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <width$}", self.value, width = self.width)
    }
}

#[allow(warnings)]
pub fn logger_format() {
    let mut builder = Builder::new();

    builder.format(|f, record| {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S.%3f");

        // 日志的消息
        let message = record.args();

        // 日志等级
        let mut style = f.style();
        let level = colored_level(&mut style, record.level());

        // 时间戳
        let time = timestamp;

        // 日志目标文件
        let target = record.target();
        let max_width = max_target_width(target);
        let mut style = f.style();
        let target = style.set_bold(true).set_color(Color::Red).value(Padded {
            value: target,
            width: max_width,
        });

        writeln!(f, "{} {} {} > {}", level, target, time, message)
    });

    // 设置日志等级过滤器
    if let Ok(s) = ::std::env::var("RUST_LOG") {
        builder.parse_filters(&s);
    }

    builder.init();
}

static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

#[allow(warnings)]
fn max_target_width(target: &str) -> usize {
    let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
    if max_width < target.len() {
        MAX_MODULE_WIDTH.store(target.len(), Ordering::Relaxed);
        target.len()
    } else {
        max_width
    }
}

#[allow(warnings)]
// 设置颜色
fn colored_level(style: &mut Style, level: Level) -> StyledValue<'_, &'static str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO"),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}

pub fn use_log() {
    // 导入
    // extern crate log;
}
