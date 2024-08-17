// my_vec! {1, 2, 3}
#[macro_export]
macro_rules! my_vec {
    // 匹配空的 Vec: let v = my_vec![];
    () => { Vec::new() };
    // 匹配(元素; 元素个数): let v = my_vec![42; 5];
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    // 匹配一个或多个表达式或者逗号
    ($($x:expr),+) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )+
            temp_vec
        }
    };
    // 匹配末尾可能的逗号
    ($($x:expr),+ $(,)?) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
