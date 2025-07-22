#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState) => {
            println!("State quarter from {:?}!", UsState);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn ks_target (number: i8) {
    match number {
        0 => println!("this is a valid bad!"),
        1 => println!("this is a valid good!"),
        _ => println!("this is a invalid number : {}", number)
    }
}

fn let_if_else (coin: Coin) {
    if let Coin::Quarter(state) = coin {
        println!("match coin quarter, the state is : {:?}", state);
    } else {
        println!("no match");
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("value_in_cents: {}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}", six);
    println!("none: {:?}", none);

    ks_target(0);
    ks_target(1);
    ks_target(-2);

    let_if_else(Coin::Penny);
    let_if_else(Coin::Quarter(UsState::Alaska));    

}