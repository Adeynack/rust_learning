fn main() {
    println!("Hello, world!");

    another_function(5, 123_456_789_012_345_678_901_234_567_890_123_456_789);

    expressions();

    println!("Five is: {}", five());
    println!("10 + 1 = {}", plus_one(10));
}

fn another_function(x: i32, y: i128) {
    println!("=== another_function");

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn expressions() {
    println!("=== Expressions");

    let x = 5;
    let y = {
        let x = 3;
        x + 1 // without ; at the end = expression vs statement.
    };
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
