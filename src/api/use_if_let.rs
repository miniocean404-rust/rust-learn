// match 的语法糖
fn show_if_let() {
    let value = Some(0u8);

    if let Some(2) = value {
        println!("two")
    } else {
        println!("other")
    }
}
