#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState)
}

fn main() {
    let penny = Coin::Penny;
    value_in_cents(penny);
    value_in_cents(Coin::Quarter(UsState::Alabama));

    ///////////////////////////////////

    let five = Some(5);
    println!("five: {:?}", five);
    let six = plus_one(five);
    println!("six: {:?}", six);
    let none = plus_one(None);
    println!("none: {:?}", none);

    ///////////////////////////////////

    let dice_roll = 9;
    match dice_roll { // We can have default impl for rest types in match
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other) //catch-all arm must be placed at last
        // _ => reroll() // if we dont want to use value of catch-all we can use _, we ignoring all other values and re rolling dice 
        _ => () // if we dont want to run any code if we get values otehr than 3 or 7, we write this
    }

    ///////////////////////////////////
    // Concise control flow with if let
    ///////////////////////////////////

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => () // Annoning boilerplate code to add, to avoid compile time error
    }

    // Better approach
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    ///////////////////////////////////
    // If we want to count all other coins except Quarter
    let mut count = 0;
    // let coin = Coin::Dime;
    let coin = Coin::Quarter(UsState::Alaska);
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1
    }
    
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!!");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // If match will miss any case, it will give compile time error
        Some(i) => Some(i+1)
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}