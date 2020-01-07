fn main() {
    if_let_with_else();
    while_let();
    for_with_deconstruction();
    let_pattern();
    function_parameters_pattern();
}

fn if_let_with_else() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let() {
    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("unstacked value {}", top);
    }
}

fn for_with_deconstruction() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn let_pattern() {
    let tuple = (1, 2, 3);

    let (x, y, z) = tuple;
    println!("x = {}, y = {}, z = {}", x, y, z);

    let (a, b, _) = tuple;
    println!("a = {}, b = {}", a, b);
}

fn print_coordinate(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn function_parameters_pattern() {
    let point = (3, 5);
    print_coordinate(&point);
}
