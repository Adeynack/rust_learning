fn main() {
    immutability();
    shadowing();
    tuples_i64litterals_destructuring();
}

fn immutability() {
    println!("Immutability");

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowing() {
    println!("Shadowing");

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

fn tuples_i64litterals_destructuring() {
    println!("Tuples / i64 Litterals / Destructuring");

    let tup = (500, 6.4, 18_390_483_290_854_i64);
    let (x, y, z) = tup;
    println!("x is {}, y is {} and z is {}", x, y, z);
}
