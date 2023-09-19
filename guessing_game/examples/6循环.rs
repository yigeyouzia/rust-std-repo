fn main() {
    // 1.while
    print!("while example:");
    let mut x = 0;
    while x != 4 {
        print!("x = {} ", x);
        x = x + 1;
    }
    println!();
    // 2.for
    print!("for example:");
    let mut idx = 0;
    let a: [i32; 5] = [1; 5]; // 初始化5个1
    for i in a.iter()
    {
        print!("a[{}] = {} ", idx, i);
        idx += 1;
    }
    // 2.1倒计时案例
    for number in (1..4).rev() {
        println!("倒计时： {}", number); // 3 2 1
    }

    // 3.loop
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    };
}