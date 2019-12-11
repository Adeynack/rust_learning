#[derive(Debug)]
enum UsState {
    NewYork,
    Alabama,
    NewMexico,
    // etc...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("A penny is worth {} cents.", value_in_cents(Coin::Penny));
    println!("A nickel is worth {} cents.", value_in_cents(Coin::Nickel));
    println!("A dime is worth {} cents.", value_in_cents(Coin::Dime));
    println!("A quarter is worth {} cents.", value_in_cents(Coin::Quarter(UsState::NewMexico)));
    println!("A quarter is worth {} cents.", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("A quarter is worth {} cents.", value_in_cents(Coin::Quarter(UsState::NewYork)));

    let v = Some(1);
    let n = None;
    println!("{:?} + 1 = {:?}", v, plus_one(&v));
    println!("{:?} + 1 = {:?}", n, plus_one(&n));

    match_placeholder();
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Oh look, a quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: &Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_placeholder() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        _ => () // does not do anything because value was not specified explicitely
    };
}
