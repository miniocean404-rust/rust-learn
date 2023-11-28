// Rust 既不能避免一个 trait 与另一个 trait 拥有相同名称的方法，也不能阻止为同一类型同时实现这两个 trait。甚至直接在类型上实现开始已经有的同名方法也是可能的！
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// 为了能够调用 Pilot trait 或 Wizard trait 的 fly 方法，我们需要使用更明显的语法以便能指定我们指的是哪个 fly 方法。这个语法展示在示例 19-18 中：
fn use_same_fn() {
    let person = Human;

    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // 为了消歧义并告诉 Rust 我们希望使用的是 Dog 的 Animal 实现，需要使用 完全限定语法( <Human as Pilot> )，这是调用函数时最为明确的方式。
    <Human as Pilot>::fly(&person)
}
