#[derive(Debug)]
enum UsStates{
    Alaaabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}

fn value_incents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {state:?}");
            25
        }
    }
}

fn plusOne(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }

    
}
fn main(){
    let five = Some(5);
    let six = plusOne(five);
    let none = plusOne(None);
}