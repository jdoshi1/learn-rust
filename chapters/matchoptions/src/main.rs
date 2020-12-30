#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!("Quarter is from State: {:?}! ", state);
            25
        }
    }
}

fn main() {
    println!("Match Coins!");
    let z = [Coin::Quarter(UsState::Alabama), Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter(UsState::Alaska)];
    for (i, c ) in z.iter().enumerate() {
        println!("Coin: {} -> Denomination: {}", i, value_in_cents(c));
    }
}
