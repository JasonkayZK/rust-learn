#[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    match_demo();
}

fn match_demo() {
    println!("Quarter value: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("Quarter value: {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}

// Bind match
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}!", state);
            25
        },
    }
}
