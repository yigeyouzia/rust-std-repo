use std::cmp::Ordering;
use std::io;
// preclude
use rand::Rng;  // trait 类似于接口

fn main() {
    println!("猜数游戏!!");
    let secret_number = rand::thread_rng().gen_range(1, 101); // 1~100 i32 u32 i64
    // 循环
    loop {
        println!("神秘数字是：{}", secret_number);
        print!("猜测一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        println!("你猜测的数是：{}", guess);

        // shadow 隐藏机制 覆盖变量
        // let guess: u32 = guess.trim().parse().expect("转换失败，请输入一个数字！");
        // match 用于枚举
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"), // arm 匹配上执行
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("woc!猜中了😋😋");
                break;
            }
        }
    }
}
