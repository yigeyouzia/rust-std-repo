fn main() {
    let x = 5;
    // 1.复杂表达式
    let y: i32 = {
        let x = 3;
        x + 1
    };
    // _func 是 空的tuple
    let _func: () = {
        let x = 3;
        x + 1;
    };
    println!("x 的值为 : {}", x); // 5
    println!("y 的值为 : {}", y); // 4

    // 2.函数嵌套
    fn five() -> i32 {
        // return 5;
        5
    }
    print!("嵌套函数的值：{}", five());
}

#[warn(dead_code)]
fn add(x: i32, y: i32) -> u32 {
    (x + y) as u32
}