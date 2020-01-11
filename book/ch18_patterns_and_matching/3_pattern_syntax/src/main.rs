fn main() {
    basic_matching();
    matching_named_variables();
    multiple_pattern();
    matching_ranges();
    destructuring_structs();
    destructuring_enums();
    destructuring_nested_structs_and_enums();
    destructuring_structs_and_tuples();
    ignoring_an_entire_value_with_underscore();
    ignoring_parts_of_a_value_with_a_nested_underscore();
    ignoring_remaining_parts_of_a_value();
    match_guards();
    bindings();
}

fn title(s: &str) {
    println!("{}{}=== {} ==={}{}",
             termion::style::Bold,
             termion::color::Fg(termion::color::LightWhite),
             s,
             termion::color::Fg(termion::color::White),
             termion::style::Reset,
    );
}

fn basic_matching() {
    title("Basic Matching");
    for x in 0..6 {
        match x {
            1 => println!("{}: one", x),
            2 => println!("{}: two", x),
            3 => println!("{}: three", x),
            _ => println!("{}: anything", x),
        }
    }
}

fn matching_named_variables() {
    title("Matching Named Variables");

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn multiple_pattern() {
    title("Multiple Pattern");

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_ranges() {
    title("Matching Ranges");

    let x = 5;

    match x {
        1..=5 => println!("one throuwh five"),
        _ => println!("something else"),
    }

    let c = 'c';
    match c {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructuring_structs() {
    title("Destructuring Structs");

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // when struct fields name match variable names
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // also possible to match
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructuring_enums() {
    title("Destructuring Enums");

    let msgs = vec![
        Message::ChangeColor(0, 160, 255),
        Message::Write("Whatever".to_string()),
        Message::Move { x: 32, y: 48 },
        Message::Quit,
    ];

    for msg in msgs {
        match msg {
            Message::Quit => println!("The Quit variant has no data to destructure."),
            Message::Move { x, y } => println!("Move in the x direction {} and in the y direction {}", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change the color to red {}, green {}, and blue {}.", r, g, b),
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum ColorMessage {
    RestoreDefaultColor,
    ChangeColor(Color),
    Bleh,
    Blah,
    Bluh,
}

fn destructuring_nested_structs_and_enums() {
    title("Destructuring Nested Structs and Enums");

    let msgs = vec![
        ColorMessage::Bleh,
        ColorMessage::Bleh,
        ColorMessage::ChangeColor(Color::Hsv(0, 160, 255)),
        ColorMessage::Bleh,
        ColorMessage::Blah,
        ColorMessage::RestoreDefaultColor,
        ColorMessage::Bluh,
        ColorMessage::Bluh,
        ColorMessage::Bluh,
        ColorMessage::ChangeColor(Color::Rgb(255, 255, 192)),
        ColorMessage::Bluh,
    ];

    for msg in msgs {
        match msg {
            ColorMessage::RestoreDefaultColor => {
                println!("Restoring defaulting color");
            }
            ColorMessage::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Changing color to red {}, green {}, and blue {}", r, g, b);
            }
            ColorMessage::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Changing color to hue {}, saturation {}, and value {}", h, s, v);
            }
            _ => {
                println!("Whatever...");
            }
        }
    }
}

fn destructuring_structs_and_tuples() {
    title("Destructuring Structs and Tuples");

    let feet_and_inches = (3, 10);
    let point = Point { x: 3, y: -10 };

    {
        let ((feet, inches), Point { x, y }) = (feet_and_inches, point);
        println!("feet = {}, inches = {}", feet, inches);
        println!("x = {}, y = {}", x, y);
    }
}

fn ignoring_an_entire_value_with_underscore() {
    title("Ignoring an Entire Value with _");

    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);
}

fn ignoring_parts_of_a_value_with_a_nested_underscore() {
    title("Ignoring Parts of a Value with a Nested _");

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }
}

fn ignoring_remaining_parts_of_a_value() {
    title("Ignoring Remaining Parts of a Value with ..");

    struct Point { x: i32, y: i32, z: i32 };

    let origin = Point { x: 0, y: 0, z: 0 };

    println!("Point has x = {}, y = {}, and z = {}", origin.x, origin.y, origin.z);
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (1, 2, 3, 4, 5, 6, 7, 8);
    match numbers {
        (first, second, .., before_last, last) => {
            println!("first is {}", first);
            println!("second is {}", second);
            println!("before last is {}", before_last);
            println!("last is {}", last);
        }
    }
}

fn match_guards() {
    title("Extra Conditionals with Match Guards");

    for num in vec![Some(1), Some(5), Some(6), None] {
        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => println!("None"),
        }
    }

    fn foo(v: Option<i32>, special_case: Option<i32>) {
        match (v, special_case) {
            (None, _) => println!("nothing to declare"),
            (Some(v), Some(s)) if v == s => println!("value {} is oh so special!", v),
            (Some(v), _) => println!("value is {}, but who cares...", v),
        }
    }

    foo(None, None);
    foo(Some(42), None);
    foo(Some(42), Some(43));
    foo(Some(42), Some(42));
    foo(None, Some(42));

    for x in 3..8 {
        for y in vec![true, false] {
            match x {
                4 | 5 | 6 if y => println!("Yes! ({}, {})", x, y),
                _ => println!("No.  ({}, {})", x, y),
            }
        }
    }
}

fn bindings() {
    title("Bindings");

    for i in 1..11 {
        let msg = Message::Move { x: i, y: 0 };
        match msg {
            Message::Move { x: x @ 3..=7, y: _ } => {
                println!("Found x in range: {}", x);
            },
            Message::Move { x: 7..=9, y: _ } => {
                println!("Found x in another range");
            },
            Message::Move { x, y: _ } => {
                println!("Found some other x: {}", x);
            },
            m => println!("Whatever: {:?}", m),
        }
    }
}
