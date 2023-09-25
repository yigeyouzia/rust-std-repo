#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// 2.附加属性  枚举变体
enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 3.枚举方法
impl IpAddrKind {
    fn call(&self) {}
}

fn main() {
    // 1.使用
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(IpAddrKind::V6);
    // 2.附加属性
    let home = IpAddrKind2::V4(127, 0, 0, 1);
    let loopback = IpAddrKind2::V6(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {}