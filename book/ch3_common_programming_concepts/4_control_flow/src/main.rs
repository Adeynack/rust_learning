fn main() {
    simple_if(2);
    simple_if(8);

    is_it_divisible(1);
    is_it_divisible(2);
    is_it_divisible(3);
    is_it_divisible(4);
    is_it_divisible(5);
    is_it_divisible(6);
    is_it_divisible(7);
    is_it_divisible(8);
    is_it_divisible(9);
    is_it_divisible(10);

    if_in_let_statement();

    repetition_with_loop();

    returning_value_from_loop();

    conditional_loops_with_while();

    looping_with_for();
}

fn simple_if(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn is_it_divisible(number: i32) {
    if number % 4 == 0 {
        println!("number {} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("number {} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("number {} is divisible by 2", number);
    } else {
        println!("number {} is not divisible by 4, 3, or 2", number);
    }
}

fn if_in_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn repetition_with_loop() {
    let mut i = 0;
    loop {
        println!("Again!");
        i += 1;
        if i > 3 {
            break;
        }
    }
}

fn returning_value_from_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}: ", result);
}

fn conditional_loops_with_while() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn looping_with_for() {
    // the manual way, using the index operator on the array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // the `for` way
    for elements in a.iter() {
        println!("the value is: {}", elements);
    }

    // countdown with ranges
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
