use procedural_macro::YourTrait;

// 运行 cargo run --example derive
#[derive(YourTrait)]
struct Person {
    #[attr1(name = "alex_name", age = 22, comment = "这是Alex")]
    pub name: String,

    pub age: u32,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    dbg!(person.name, person.age);
}
