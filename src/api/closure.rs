use std::{collections::HashMap, thread, time::Duration};

pub fn use_exec_closure() {
    let x = 32;

    // 闭包可不标注参数类型，因为在函数作用域内，编译器可以自己识别出类型
    // 但是执行时候传入的参数会决定它的类型，并且与它绑定，再次调用传入其他类型会报错
    // 直接返回值的闭包：let direct_return = |num| num
    let closure = |num| {
        println!("函数正在执行");

        // 睡眠 2 秒
        thread::sleep(Duration::from_secs(2));

        // 捕获值的方式
        // 1. 取得所有权 FnOnce 所有的闭包都实现了 FnOnce 这个 Trait
        // 2. 可变借用 FnMut 没有移动捕获变量的实现了FnMut
        // 3. 不可变借用 Fn 无需可变访问捕获变量的闭包实现了 Fn
        println!(
            "闭包可以捕获外部的参数（但是会有额外的内存开销），函数不可以{}",
            x
        );

        num
    };

    let mut cache_closure = Cache::new(closure);

    println!("闭包执行结果 {}", cache_closure.value(2));
    println!("闭包执行结果 {}", cache_closure.value(3));
}

// 将结构体和闭包绑定
// 这个模式通常叫记忆化或延迟计算：
//  只会在需要结果时才执行闭包
//  可缓存结果
struct Cache<T>
where
    // 使用 trait 时候，优先使用 Fn
    T: Fn(u32) -> u32,
{
    closure: T,
    hash: HashMap<u32, u32>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(closure: T) -> Cache<T> {
        let hash: HashMap<u32, u32> = HashMap::new();

        Cache { closure, hash }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.hash.get(&arg) {
            Some(v) => *v,
            None => {
                let res = (self.closure)(arg);
                self.hash.insert(arg, res);

                res
            }
        }
    }
}

// move: 参数列表前使用 move 关键字，可以使闭包获取他所使用环境的所有权
// 闭包传递给新线程，将数据归于新线程所有时，这个最有用
pub fn use_move_closure() {
    let x = vec![1, 2];

    let closure = move |z: Vec<i32>| z == x;

    let y = vec![1, 2];

    closure(y);

    // println!("所有权消失 {:?}", x)
}
