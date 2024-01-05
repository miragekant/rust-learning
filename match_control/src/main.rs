#[derive(Debug)]
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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state: {:?}", state);
            25
        }
    }
}

// Matches are exhaustive
// We must exhaust every possibility in order
// for the code to be valid
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

/*
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
    // if you don't need the value
    _ => rerol(),
    // with unit value
    _ => (), // do nothing
}
*/


fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
    plus_one(Some(5));
    plus_one(None);
}
