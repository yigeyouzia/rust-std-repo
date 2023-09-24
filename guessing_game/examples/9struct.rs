// 1.定义
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 3.简写 类似js
fn build_user(email: String, username: String) -> User {
    User {
        username, // 简写
        email,
        sign_in_count: 0,
        active: false,
    }
}

// 5.tuple struct
struct RGB(i32, i32, i32);

fn main() {
    // 2.声明 ， user1可变，所有字段可变， 不允许部分不变
    let mut user1 = User {
        username: String::from("cyt"),
        email: String::from("cyt@qq.com"),
        active: true,
        sign_in_count: 556,
    };
    user1.sign_in_count = 600;
    // 4. 扩展运算符 类似js
    let user2 = User {
        sign_in_count: 0,
        active: false,
        ..user1
    };
    print!("{}", user2.username); // cyt
    // 5.tuple struct
    let black = RGB(0, 0, 0);
}