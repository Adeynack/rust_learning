fn main() {
    array_type();
}

fn array_type() {
    let a = [1, 2, 3, 4, 5]; //  inferred to [i32;_]
    let b: [i64; 5] = [1, 2, 3, 4, 5]; // explicitely typed
    println!("Array in normal print: {:?}", a);
    println!("Array in pretty print: {:#?}", b);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Months: {:?}", months);

    let c = [3; 5];
    println!("Value-initialized array: {:?}", c);

    let first_a = a[0];
    let second_a = a[1];
    println!("The first values of 'a' are {} and {}", first_a, second_a);

    // println!("{}", a[6]); // that does not even compile, because the index is out of bound, detected at compilation time.

    let index = 6;
    // println!("{}", a[index]); // that fails at runtime, since the index is dynamic
}
