use std::any;
use std::any::Any;

// 1.常量
const MAX_POINTS: u32 = 100_00;

fn main() {
    let x = 5;
    // x = x + 1; 报错
    let x = x + 1; // 2.shadowing
    let spaces = "123"; // 字符串
    let spaces = 1; // 整数

    // 推断类型 指定类型
    let guess: u32 = "42  ".trim().parse().expect("not a number");
    print!("guess: {}", guess);
    // 3.标量类型
    let a: u8 = 255;
    let c_float = 2.0;
    let guess: bool = true;
    let d_char: char = '😋';
    // 4.复合类型 tuple和数组 长度都是固定的
    let _tup: (u32, u64, f32) = (1, 2, 3.0); // 元组 tuple
    let (x, y, z) = _tup; // 解包
    println!("访问tup元素： {}, {}, {}", _tup.0, _tup.1, _tup.2);

    let _arr: [i32; 5] = [1, 2, 3, 4, 5];
    let _arr1: [i32; 5] = [3; 5];
    print!("{}", _arr[0]);
}
