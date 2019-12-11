fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    Message::ChangeColor(123, 251, 35).call();
    Message::Write(String::from("Bleh")).call();
    Message::Quit.call();
    Message::Move { x: 32, y: 48 }.call();

    using_option();
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing {:?}", ip_kind);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Called {:?}", self);
    }
}

// todo: Understand how to define abstract methods in enums
// impl Message::Quit {
//     fn call(&self) {
//         println!("It's over...");
//     }
// }

fn using_option() {
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("a string");

    let absent_number: Option<i32> = None;

    println!("some_number = {:?}", some_number);
    println!("some_string = {:?}", some_string);
    println!("absent_number = {:?}", absent_number);
}
