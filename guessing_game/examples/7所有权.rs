fn main() {
    // s1 s2 5 都在栈内存 合理
    let s1: i32 = 5;
    let s2 = s1;

    // 1.所有权转让 字符串在堆内存 s1 s2在栈
    let s1: String = String::from("cyt");
    // let s2: String = s1;
    // println!("s1 不存在 : {}",s1);
    let s2: String = s1.clone();
    println!("s1 存在 : {}", s1);
    println!("s2 : {}", s2);

    let s1 = String::from("hello");
    let s2 = &s1;
    // let s3 = s1; 将s1所有权给s3  s2将失效
    println!("s1 is {}, s2 is {}", s1, s2);

    // 正确 s2 为mut 可将指向改变
    let s1 = String::from("hello22");
    let mut s2 = &s1;
    let s3 = s1;
    s2 = &s3; // 重新从 s3 租借所有权
    println!("{}", s2);

    // 2.可变引用
    // 2.1错误演示
    println!("2.1错误演示：");
    let s1 = String::from("run");
    let s2 = &s1;
    println!("{}", s2);
    // s2.push_str("oob"); // 错误，禁止修改租借的值
    println!("{}", s2);
    // 2.2正确演示
    println!("2.2正确演示");
    let mut s1 = String::from("run"); // s1 是可变的
    let s2 = &mut s1; // s2 是可变的引用
    s2.push_str("oob");
    // println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    // 2.3 可变引用不可以重用
    let mut s = String::from("hello");
    // let r1 = &mut s; 可变引用与不可变引用相比除了权限不同以外，可变引用不允许多重引用，但不可变引用可以：
    let r2 = &mut s;
    println!("{}", r2);

    let t1: String = String::from("test str");
    let t2: String = take_adn_give_back(t1);
    println!("t1 {}", t1);
}

fn take_adn_give_back(a_string: String) -> String {
    a_string
}