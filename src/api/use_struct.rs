// 用于展示结构体，得到 Debug 派生

// 必须为结构体显式的声明 Debug 功能，主要是使得结构体派生于 Debug
// derive 派生
#[derive(Debug)]
struct Rect {
    width: i8,
    height: i8,
}

impl Rect {
    fn get_area(&self) -> i8 {
        &self.width + &self.height
    }
}

pub fn format_struct_example() {
    let show_object = Rect {
        width: 12,
        height: 12,
    };

    println!("#?: {:#?}", show_object);
    println!("? : {:?}", show_object);
}

pub fn get_area() {
    let show_object = Rect {
        width: 8,
        height: 8,
    };

    let area: i8 = show_object.get_area();

    println!("宽度：{}", show_object.width);
    println!("高度：{}", show_object.height);
    println!("面积：{}", area)
}
