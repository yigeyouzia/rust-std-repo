#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    penny,
    Nickel,
    Dime,
    Quarter(UsState), // 关联上面的enum
}

fn value_in_cents(coin: Coin) -> u8 {
    // 1.模式匹配
    match coin {
        Coin::penny => {
            println!("匹配到：penny！");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    print!("{}", value_in_cents(c));
    // State quarter from Alaska!
    // 25
    // 2.if let
    let v = Some(3u8);
    if let Some(3) = v {
        println!("tree")
    } else {
        println!("others")
    }
}