use anyhow::Result;
use macros::my_vec;

fn main() -> Result<()> {
    let v: Vec<i32> = my_vec!["193".parse()?, 21, 33, "42".parse()?, 57,];
    println!("{:?}", v);

    // 创建 Vec 的另外一种形式
    let v1 = <[_]>::into_vec(Box::new([1, 2, 3, 4]));
    println!("{:?}", v1);
    Ok(())
}
