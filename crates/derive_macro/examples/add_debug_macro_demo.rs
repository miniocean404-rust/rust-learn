// 运行 cargo r -p derive_macro --example derive_macro

use derive_macro::PersonMacro;

#[derive(PersonMacro)]
struct Person {
    #[attr1(name = "alex_name", age = 22, comment = "这是 Alex")]
    name: String,

    #[attr2]
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{:?} {:?}", person.get_name(), person.get_age());
}