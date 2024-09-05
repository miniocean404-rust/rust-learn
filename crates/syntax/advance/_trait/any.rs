// 动态 Trait 转为指定类型
// 动态 Trait 介绍 oo -> dyn_trait

use std::{any::Any, error::Error, fmt::Debug, sync::Arc};

// 最好不要实现 Copy 和 Clone，因为它们返回 Self 导致转化不是一个 object-safe trait
pub trait Mode: Any + Debug {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct DevlopmentMode {
    name: String,
}

impl Mode for DevlopmentMode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct TestingMode {
    name: String,
}

impl Mode for TestingMode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn use_box_mode<M: Mode + 'static>(mode: M) -> Result<(), Box<dyn Error>> {
    let mode = Box::new(mode) as Box<dyn Any>;

    if mode.is::<DevlopmentMode>() {
        let dev = mode.downcast::<DevlopmentMode>().unwrap();
        println!("Development mode: {}", dev.name);
    } else {
        let test = mode.downcast::<TestingMode>().unwrap();
        println!("Testing mode: {}", test.name);
    }

    Ok(())
}

pub fn use_arc_mode<M: Mode + 'static>(mode: M) -> Result<(), Box<dyn Error>> {
    let mode = Arc::new(mode) as Arc<dyn Any>;

    if mode.is::<DevlopmentMode>() {
        let dev = mode
            .downcast_ref::<DevlopmentMode>()
            .ok_or("Failed to downcast to DevlopmentMode")?;

        println!("Development mode: {}", dev.name);
    } else {
        let test = mode
            .downcast_ref::<TestingMode>()
            .ok_or("Failed to downcast to TestingMode")?;

        println!("Testing mode: {}", test.name);
    }

    Ok(())
}

pub fn print_if_string(value: &dyn Any) {
    if let Some(s) = value.downcast_ref::<String>() {
        println!("String ({}): {}", s.len(), s);
    } else {
        println!("Not a string");
    }
}
