#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

// 2.struct方法
impl Rectangle {
    fn get_area(&self) -> u32 {
        self.length * self.width
    }
    // 判断是否能容纳另一个长方形
    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
    // 3.关联函数
    // 一般用于构造函数
    // 模块命名空间
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    let rec = Rectangle {
        width: 100,
        length: 200,
    };
    // 面积
    println!("area:");
    println!("{}", area(&rec)); // 20000 加上引用 防止所有权给area函数
    println!("{}", rec.length); // 所有权仍在main函数
    println!("{}", rec.get_area());
    // 打印
    println!("{:?}", rec); // Rectangle { width: 100, length: 200 }
    println!("{:#?}", rec); // Rectangle { width: 100, length: 200 }
    /*
        Rectangle {
            width: 100,
            length: 200,
        }
    */
    // 3.关联函数
    let s = Rectangle::square(20);
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.length
}