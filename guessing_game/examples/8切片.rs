fn main() {
    // 1.切片的返回类型都是 &str  str和String都是字符串类型
    let mut s = String::from("runoob");
    let slice = &s[0..3];
    // s.push_str("yes!"); // 错误 不能修改切片引用
    println!("slice = {}", slice);
    // 事已至此我们必须分辨这两者概念的区别了。在 Rust 中有两种常用的字符串类型：str 和 String。
    // str 是 Rust 核心语言类型，就是本章一直在讲的字符串切片（String Slice），常常以引用的形式出现（&str）。
    // 凡是用双引号包括的字符串常量整体的类型性质都是 &str：

    // 2.有一个快速的办法可以将 String 转换成 &str：
    let s1 = String::from("hello");
    let s2 = &s1[..];

    // 3.非字符串切片
    // 除了字符串以外，其他一些线性数据结构也支持切片操作，例如数组：
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i); // 1 3 5
    }
}