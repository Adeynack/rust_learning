fn main() {
    function_pointers();
    returning_functions();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn function_pointers() {
    let answer1 = do_twice(add_one, 5);
    println!("The answer #1 is: {}", answer1);

    let closure = |a| a * 2;
    let answer2 = do_twice(closure, 5);
    println!("The answer #2 is: {}", answer2);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string) // instead of .map(|i| i.to_string())
        .collect();
    println!("Mapped numbers are: {:?}", list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let mut list_of_statuses: Vec<Status> = (0u32..20)
        .map(|i| Status::Value(i)) // .map(Status::Value)
        .collect();
    let stops = vec![Status::Stop];
    list_of_statuses.extend(stops);
    println!("List of statuses: {:?}", list_of_statuses);
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returning_functions() {
    let _f = returns_closure();
}
