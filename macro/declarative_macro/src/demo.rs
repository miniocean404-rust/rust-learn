#[macro_export]
macro_rules! demo1 {
    ($name:expr) => {
        println!("一个参数：{:?} = {:?}", stringify!($expr), $key1);
    };

    ($key1:expr,$key2:expr) => {
        println!(
            "两个参数：{:?} = {:?} {:?}",
            stringify!($expr),
            $key1,
            $key2
        );
    };
}

#[macro_export]
macro_rules! map {
    ($( $key:expr => $value:expr ),*) => {
        let mut hm = HashMap::new();
        // $()*，它的意思是其中的代码会重复展开为多次一样的代码，不一样的参数
        $(hm.insert( $key, $value);)*
    };

    ($( $key:expr => $value:expr ),*) => {
        $(println!("{},{}", $key, $value);)*
    };
}

#[macro_export]
macro_rules! sum {
    ($name:expr) => {
        $name
    };

    ($x:expr, $($arg:expr),*) => {
        $x + sum!($($arg),*)
    };

    ( $( $arg:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($arg);
            )*
            println!("{:?}",temp_vec);
        }
    };
}
