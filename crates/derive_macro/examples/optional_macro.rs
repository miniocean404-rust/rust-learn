use derive_macro::Optional;

// 运行 cargo r -p derive_macro --example optional_macro

#[allow(dead_code)]
#[derive(Optional, Debug)]
struct User {
    #[optional(rename = value)]
    pub id: usize,
    pub subs: Option<Vec<User>>,
    #[optional(skip)]
    pub descriptions: String,
}

fn main() {
    // 通过宏生成的 OptionalUser
    let test = OptionalUser {
        value: None,
        subs: None,
        descriptions: String::from("这是描述"),
    };
    dbg!(test.subs,test.value,test.descriptions);
}