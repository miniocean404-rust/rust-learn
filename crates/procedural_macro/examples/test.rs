use procedural_macro::my_attribute;

fn main() {}

#[my_attribute(hello)]
fn use_attribute_derive() {
    println!("函数本身的调用")
}
