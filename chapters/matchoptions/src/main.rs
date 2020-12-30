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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Non-exhaustive function. Needs None arm.
// fn plus_one_more(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }

fn main() {
    println!("Match Coins!");
    let z = [Coin::Quarter(UsState::Alabama), Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter(UsState::Alaska)];
    for (i, c) in z.iter().enumerate() {
        let x = value_in_cents(c);
        println!("Coin: {} -> Denomination: {}", i, x);
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five:{:?} six:{:?} None:{:?}", five, six, none);
}
